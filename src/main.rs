mod user;

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;

#[derive(Parser)]
#[command(name = "userkit")]
#[command(about = "A CLI tool to manage users in Unix-based OS", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  List {
    #[clap(long, short, default_value = "table")]
    format: ListFormat,
  },
  Info {
    username: String,
  },
  Add {
    username: String,
    home_dir: String,
  },
  Delete {
    username: String,
  },
  Modify {
    username: String,
  },
}

#[derive(ValueEnum, Clone)]
enum ListFormat {
  Table,
  Json,
}

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Commands::List { format } => match format {
      ListFormat::Table => list_users_as_table(),
      ListFormat::Json => list_users_as_json(),
    },
    Commands::Info { username } => user_info(username),
    Commands::Add { username, home_dir } => add_user(username, home_dir),
    Commands::Delete { username } => delete_user(username),
    Commands::Modify { username } => println!("Modify user {}", username),
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
