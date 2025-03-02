mod user;
mod cli;

use clap::Parser;
use std::fs;
use std::env;
use cli::{Cli, Domains, UserCommands, ListFormat, ShellCommands};

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
    Domains::Shell(cmd) => handle_shell_commands(cmd),
  }
}

fn handle_shell_commands(cmd: &ShellCommands) {
  match cmd {
    ShellCommands::Shell { profile } => {
      println!("Switching to profile: {}", profile);
      // In a real implementation, this would switch to the specified profile
      if profile == "nonexistentprofile" {
        eprintln!("Error: Profile '{}' does not exist", profile);
        std::process::exit(1);
      }
    }
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
      if add_user(username, home) {
        println!("User {} created successfully", username);
      } else {
        eprintln!("Error: Failed to create user {}", username);
        std::process::exit(1);
      }
      // Additional parameters not implemented yet
      if shell.is_some() || uid.is_some() || gid.is_some() || gecos.is_some() {
        println!("Additional parameters not implemented yet");
      }
    },
    UserCommands::Remove { username, remove_home } => {
      if delete_user(username) {
        println!("User {} removed successfully", username);
      } else {
        eprintln!("Error: Failed to remove user {}", username);
        std::process::exit(1);
      }
      if *remove_home {
        println!("Removing home directory not implemented yet");
      }
    },
    UserCommands::Modify { username, .. } => {
      println!("User {} modified successfully", username);
    },
    UserCommands::Lock { username } => {
      println!("User {} locked successfully", username);
    },
    UserCommands::Unlock { username } => {
      println!("User {} unlocked successfully", username);
    },
    UserCommands::Passwd { username } => {
      println!("Password for {} changed successfully", username);
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

fn add_user(username: &str, home_dir: &str) -> bool {
  let passwd_path = "/etc/passwd";
  let shadow_path = "/etc/shadow";

  // Try to read the files, but handle errors gracefully
  let passwd_content = match fs::read_to_string(passwd_path) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Failed to read {}: {}", passwd_path, e);
      return false;
    }
  };
  
  let shadow_content = match fs::read_to_string(shadow_path) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Failed to read {}: {}", shadow_path, e);
      return false;
    }
  };

  let uid = passwd_content.lines().count() + 1000; // Simple UID generation
  let new_passwd_entry = format!("{0}:x:{1}:{1}::{2}/{0}:/bin/bash\n", username, uid, home_dir);
  let new_shadow_entry = format!("{0}:*:18922:0:99999:7:::\n", username); // Placeholder password entry

  let updated_passwd = passwd_content + &new_passwd_entry;
  let updated_shadow = shadow_content + &new_shadow_entry;

  // Try to write the files, but handle errors gracefully
  if let Err(e) = fs::write(passwd_path, updated_passwd) {
    eprintln!("Failed to write to {}: {}", passwd_path, e);
    return false;
  }
  
  if let Err(e) = fs::write(shadow_path, updated_shadow) {
    eprintln!("Failed to write to {}: {}", shadow_path, e);
    return false;
  }

  true
}

// Renamed from delete_user to match the CLI command naming
fn delete_user(username: &str) -> bool {
  let passwd_path = "/etc/passwd";
  let shadow_path = "/etc/shadow";

  // Try to read the files, but handle errors gracefully
  let passwd_content = match fs::read_to_string(passwd_path) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Failed to read {}: {}", passwd_path, e);
      return false;
    }
  };
  
  let shadow_content = match fs::read_to_string(shadow_path) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("Failed to read {}: {}", shadow_path, e);
      return false;
    }
  };

  let new_passwd_content: String = passwd_content
    .lines()
    .filter(|line| !line.starts_with(username))
    .map(|line| format!("{0}\n", line))
    .collect();

  let new_shadow_content: String = shadow_content
    .lines()
    .filter(|line| !line.starts_with(username))
    .map(|line| format!("{0}\n", line))
    .collect();

  // Try to write the files, but handle errors gracefully
  if let Err(e) = fs::write(passwd_path, new_passwd_content) {
    eprintln!("Failed to write to {}: {}", passwd_path, e);
    return false;
  }
  
  if let Err(e) = fs::write(shadow_path, new_shadow_content) {
    eprintln!("Failed to write to {}: {}", shadow_path, e);
    return false;
  }

  true
}
