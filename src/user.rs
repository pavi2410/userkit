use std::fs;

pub(crate) struct User {
  pub(crate) username: String,
  #[allow(dead_code)]
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
