use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn run_userkit_command(subcommands: Vec<&str>, run_privileged: bool) -> Command {
  let mut cmd = if run_privileged {
    Command::new("sudo")
  } else {
    Command::new("cargo")
  };

  if run_privileged {
    cmd.arg("cargo");
  }

  cmd.arg("run").arg("--");

  for subcmd in subcommands {
    cmd.arg(subcmd);
  }

  cmd
}

#[test]
fn test_user_add() {
  let mut cmd = run_userkit_command(vec!["user", "add", "testuser"], true);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser created"));
}

#[test]
fn test_user_add_with_options() {
  let mut cmd = run_userkit_command(
    vec![
      "user",
      "add",
      "testuser2",
      "--home-dir",
      "/home/testuser2",
      "--shell",
      "/bin/bash",
    ],
    true,
  );

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser2 created"));
}

#[test]
fn test_user_list() {
  let mut cmd = run_userkit_command(vec!["user", "list"], false);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Username"));
}

#[test]
fn test_user_list_json_format() {
  let mut cmd = run_userkit_command(vec!["user", "list", "--format", "json"], false);

  cmd.assert().success().stdout(predicate::str::contains("["));
}

#[test]
fn test_user_info_existing() {
  let mut cmd = run_userkit_command(vec!["user", "info", "root"], false); // Using root user which is guaranteed to exist

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Username: root"));
}

#[test]
fn test_user_info_nonexistent() {
  let mut cmd = run_userkit_command(vec!["user", "info", "nonexistentuser"], false); // Using a user that definitely doesn't exist

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_user_remove() {
  let mut cmd = run_userkit_command(vec!["user", "remove", "testuser"], true);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser removed"));
}

#[test]
fn test_user_remove_with_home() {
  let mut cmd = run_userkit_command(vec!["user", "remove", "testuser2", "--remove-home"], true);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser2 removed"));
}

#[test]
fn test_user_modify() {
  let mut cmd = run_userkit_command(
    vec!["user", "modify", "testuser", "--shell", "/bin/zsh"],
    true,
  );

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser modified"));
}

#[test]
fn test_user_lock() {
  let mut cmd = run_userkit_command(vec!["user", "lock", "testuser"], true);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser locked"));
}

#[test]
fn test_user_unlock() {
  let mut cmd = run_userkit_command(vec!["user", "unlock", "testuser"], true);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser unlocked"));
}

#[test]
fn test_user_passwd() {
  let mut cmd = run_userkit_command(vec!["user", "passwd", "testuser"], true);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Password for testuser changed"));
}

#[test]
fn test_user_add_invalid() {
  let mut cmd = run_userkit_command(vec!["user", "add", "root"], true); // Trying to add a user that likely already exists

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
