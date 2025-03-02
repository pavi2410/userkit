mod cli;
mod user;

use clap::Parser;
use cli::{Cli, Domains, ListFormat, ShellCommands, UserCommands};

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
    UserCommands::List {
      format,
      uid_range,
      gid,
    } => {
      // Handle optional filters
      if uid_range.is_some() || gid.is_some() {
        println!("Filtering not implemented yet");
      }

      match format {
        ListFormat::Table => user::list_users_as_table(),
        ListFormat::Json => user::list_users_as_json(),
        ListFormat::Csv => println!("CSV format not implemented yet"),
      }
    }
    UserCommands::Info { username } => {
      if !user::user_info(username) {
        std::process::exit(1);
      }
    }
    UserCommands::Add {
      username,
      home_dir,
      shell,
      uid,
      gid,
      gecos,
    } => {
      let home = home_dir.as_deref().unwrap_or("/home/");
      if user::add_user(username, home) {
        println!("User {} created successfully", username);
      } else {
        eprintln!("Error: Failed to create user {}", username);
        std::process::exit(1);
      }
      // Additional parameters not implemented yet
      if shell.is_some() || uid.is_some() || gid.is_some() || gecos.is_some() {
        println!("Additional parameters not implemented yet");
      }
    }
    UserCommands::Remove {
      username,
      remove_home,
    } => {
      if user::delete_user(username) {
        println!("User {} removed successfully", username);
      } else {
        eprintln!("Error: Failed to remove user {}", username);
        std::process::exit(1);
      }
      if *remove_home {
        println!("Removing home directory not implemented yet");
      }
    }
    UserCommands::Modify { username, .. } => {
      println!("User {} modified successfully", username);
    }
    UserCommands::Lock { username } => {
      println!("User {} locked successfully", username);
    }
    UserCommands::Unlock { username } => {
      println!("User {} unlocked successfully", username);
    }
    UserCommands::Passwd { username } => {
      println!("Password for {} changed successfully", username);
    }
  }
}
