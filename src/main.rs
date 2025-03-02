mod user;
mod cli;

use clap::Parser;
use std::fs;
use cli::{Cli, Domains, UserCommands, ListFormat};

// CLI structure is now defined in cli.rs

fn main() {
  let cli = Cli::parse();

  match &cli.domain {
    Domains::User(cmd) => handle_user_commands(cmd),
    Domains::Group(_) => println!("Group management not implemented yet"),
    Domains::Perm(_) => println!("Permission management not implemented yet"),
    Domains::Role(_) => println!("Role management not implemented yet"),
    Domains::Guest(_) => println!("Guest account management not implemented yet"),
    Domains::Config(_) => println!("Configuration management not implemented yet"),
    Domains::Shell(_) => println!("Shell access not implemented yet"),
  }
}

fn handle_user_commands(cmd: &UserCommands) {
  match cmd {
    UserCommands::List { format, uid_range, gid } => {
      // Handle optional filters
      if uid_range.is_some() || gid.is_some() {
        println!("Filtering not implemented yet");
      }
      
      match format {
        ListFormat::Table => list_users_as_table(),
        ListFormat::Json => list_users_as_json(),
        ListFormat::Csv => println!("CSV format not implemented yet"),
      }
    },
    UserCommands::Info { username } => user_info(username),
    UserCommands::Add { username, home_dir, shell, uid, gid, gecos } => {
      let home = home_dir.as_deref().unwrap_or("/home/");
      add_user(username, home);
      // Additional parameters not implemented yet
      if shell.is_some() || uid.is_some() || gid.is_some() || gecos.is_some() {
        println!("Additional parameters not implemented yet");
      }
    },
    UserCommands::Remove { username, remove_home } => {
      delete_user(username);
      if *remove_home {
        println!("Removing home directory not implemented yet");
      }
    },
    UserCommands::Modify { username, .. } => {
      println!("Modifying user {} not implemented yet", username);
    },
    UserCommands::Lock { username } => {
      println!("Locking user {} not implemented yet", username);
    },
    UserCommands::Unlock { username } => {
      println!("Unlocking user {} not implemented yet", username);
    },
    UserCommands::Passwd { username } => {
      println!("Changing password for user {} not implemented yet", username);
    },
  }
}

fn list_users_as_table() {
  let users = user::list_users();
  println!(
    "{: <20} {: >10} {: >10} {: <20} {: <20}",
    "Username", "UID", "GID", "Home Directory", "Shell"
  );
  println!("{}", "-".repeat(84));
  for user in users {
    println!(
      "{: <20} {: >10} {: >10} {: <20} {: <20}",
      user.username, user.uid, user.gid, user.home_dir, user.shell
    );
  }
}

fn list_users_as_json() {
  let users = user::list_users();
  let mut json = String::from("[\n");
  for (i, user) in users.iter().enumerate() {
    json.push_str(&format!(
            "  {{ \"username\": \"{}\", \"uid\": {}, \"gid\": {}, \"home_dir\": \"{}\", \"shell\": \"{}\" }}",
            user.username, user.uid, user.gid, user.home_dir, user.shell
        ));
    if i < users.len() - 1 {
      json.push_str(",\n");
    }
  }
  json.push_str("\n]");
  println!("{}", json);
}

fn user_info(username: &str) {
  let users = user::list_users();
  for user in users {
    if user.username == username {
      println!("Username: {}", user.username);
      println!("UID: {}", user.uid);
      println!("GID: {}", user.gid);
      println!("GECOS: {}", user.gecos);
      println!("Home Directory: {}", user.home_dir);
      println!("Shell: {}", user.shell);
      return;
    }
  }
  println!("User {} not found", username);
}

fn add_user(username: &str, home_dir: &str) {
  let passwd_path = "/etc/passwd";
  let shadow_path = "/etc/shadow";

  let mut passwd_content = fs::read_to_string(passwd_path).expect("Failed to read /etc/passwd");
  let mut shadow_content = fs::read_to_string(shadow_path).expect("Failed to read /etc/shadow");

  let uid = passwd_content.lines().count() + 1000; // Simple UID generation
  let new_passwd_entry = format!("{}:x:{}:{}::{}:/bin/bash\n", username, uid, uid, home_dir);
  let new_shadow_entry = format!("{}:*:18922:0:99999:7:::\n", username); // Placeholder password entry

  passwd_content.push_str(&new_passwd_entry);
  shadow_content.push_str(&new_shadow_entry);

  fs::write(passwd_path, passwd_content).expect("Failed to write to /etc/passwd");
  fs::write(shadow_path, shadow_content).expect("Failed to write to /etc/shadow");

  println!("User {} added successfully", username);
}

// Renamed from delete_user to match the CLI command naming
fn delete_user(username: &str) {
  let passwd_path = "/etc/passwd";
  let shadow_path = "/etc/shadow";

  let passwd_content = fs::read_to_string(passwd_path).expect("Failed to read /etc/passwd");
  let shadow_content = fs::read_to_string(shadow_path).expect("Failed to read /etc/shadow");

  let new_passwd_content: String = passwd_content
    .lines()
    .filter(|line| !line.starts_with(username))
    .map(|line| format!("{}\n", line))
    .collect();

  let new_shadow_content: String = shadow_content
    .lines()
    .filter(|line| !line.starts_with(username))
    .map(|line| format!("{}\n", line))
    .collect();

  fs::write(passwd_path, new_passwd_content).expect("Failed to write to /etc/passwd");
  fs::write(shadow_path, new_shadow_content).expect("Failed to write to /etc/shadow");

  println!("User {} deleted successfully", username);
}
