use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn run_userkit_command(subcommands: Vec<&str>) -> Command {
  let mut cmd = Command::new("./target/debug/userkit");

  for subcmd in subcommands {
    cmd.arg(subcmd);
  }

  cmd
}

fn sudo_run_userkit_command(subcommands: Vec<&str>) -> Command {
  let mut cmd = Command::new("sudo");
  cmd.arg("./target/debug/userkit");

  for subcmd in subcommands {
    cmd.arg(subcmd);
  }

  cmd
}

#[test]
fn test_user_add() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "add", "testuser"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser created"));
}

#[test]
fn test_user_add_with_options() {
  let mut cmd = sudo_run_userkit_command(vec![
    "user",
    "add",
    "testuser2",
    "--home-dir",
    "/home/testuser2",
    "--shell",
    "/bin/bash",
  ]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser2 created"));
}

#[test]
fn test_user_list() {
  let mut cmd = run_userkit_command(vec!["user", "list"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Username"));
}

#[test]
fn test_user_list_json_format() {
  let mut cmd = run_userkit_command(vec!["user", "list", "--format", "json"]);

  cmd.assert().success().stdout(predicate::str::contains("["));
}

#[test]
fn test_user_info_existing() {
  let mut cmd = run_userkit_command(vec!["user", "info", "root"]); // Using root user which is guaranteed to exist

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Username: root"));
}

#[test]
fn test_user_info_nonexistent() {
  let mut cmd = run_userkit_command(vec!["user", "info", "nonexistentuser"]); // Using a user that definitely doesn't exist

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_user_remove() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "remove", "testuser"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser removed"));
}

#[test]
fn test_user_remove_with_home() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "remove", "testuser2", "--remove-home"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser2 removed"));
}

#[test]
fn test_user_modify() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "modify", "testuser", "--shell", "/bin/zsh"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser modified"));
}

#[test]
fn test_user_lock() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "lock", "testuser"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser locked"));
}

#[test]
fn test_user_unlock() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "unlock", "testuser"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser unlocked"));
}

#[test]
fn test_user_passwd() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "passwd", "testuser"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Password for testuser changed"));
}

#[test]
fn test_user_add_invalid() {
  let mut cmd = sudo_run_userkit_command(vec!["user", "add", "root"]); // Trying to add a user that likely already exists

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
