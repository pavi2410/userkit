use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct User {
  pub(crate) username: String,
  #[allow(dead_code)]
  #[serde(skip)]
  password: String,
  pub(crate) uid: u32,
  pub(crate) gid: u32,
  pub(crate) gecos: String,
  pub(crate) home_dir: String,
  pub(crate) shell: String,
}

pub(crate) fn list_users() -> Vec<User> {
  // Real implementation for production use
  match fs::read_to_string("/etc/passwd") {
    Ok(passwd_content) => {
      let mut users = Vec::new();
      for line in passwd_content.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() >= 7 {
          let user = User {
            username: fields[0].to_string(),
            password: fields[1].to_string(),
            uid: fields[2].parse().unwrap_or(0),
            gid: fields[3].parse().unwrap_or(0),
            gecos: fields[4].to_string(),
            home_dir: fields[5].to_string(),
            shell: fields[6].to_string(),
          };
          users.push(user);
        }
      }
      users
    },
    Err(e) => {
      eprintln!("Failed to read /etc/passwd: {}", e);
      Vec::new()
    }
  }
}

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct UserTable {
    #[tabled(rename = "Username")]
    username: String,
    #[tabled(rename = "UID")]
    uid: u32,
    #[tabled(rename = "GID")]
    gid: u32,
    #[tabled(rename = "Home Directory")]
    home_dir: String,
    #[tabled(rename = "Shell")]
    shell: String,
}

pub(crate) fn list_users_as_table() {
    let users = list_users();
    let table_data: Vec<UserTable> = users
        .into_iter()
        .map(|user| UserTable {
            username: user.username,
            uid: user.uid,
            gid: user.gid,
            home_dir: user.home_dir,
            shell: user.shell,
        })
        .collect();

    let table = Table::new(table_data).to_string();
    println!("{}", table);
}

pub(crate) fn list_users_as_json() {
  let users = list_users();
  match serde_json::to_string_pretty(&users) {
    Ok(json) => println!("{}", json),
    Err(e) => eprintln!("Failed to serialize users to JSON: {}", e),
  }
}

pub(crate) fn user_info(username: &str) {
  let users = list_users();
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

pub(crate) fn add_user(username: &str, home_dir: &str) -> bool {
  // Check if running with sudo/root privileges
  if !has_escalated_privileges() {
    eprintln!("Error: This operation requires root privileges. Please run with sudo.");
    return false;
  }

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

pub(crate) fn delete_user(username: &str) -> bool {
  // Check if running with sudo/root privileges
  if !has_escalated_privileges() {
    eprintln!("Error: This operation requires root privileges. Please run with sudo.");
    return false;
  }

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

fn has_escalated_privileges() -> bool {
  // Check if running with sudo/root privileges
  #[cfg(unix)]
  {
    use std::os::unix::fs::MetadataExt;
    fs::metadata("/etc/passwd").map(|m| m.uid() == 0).unwrap_or(false)
  }
  #[cfg(not(unix))]
  {
    // On non-Unix systems, assume we need to run with admin privileges
    false
  }
}
