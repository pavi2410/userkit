use assert_cmd::prelude::*;
use predicates::prelude::*;
mod test_utils;
use test_utils::run_userkit_command;

#[test]
fn test_guest_create() {
  let mut cmd = run_userkit_command(vec!["guest", "create"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account created"));
}

#[test]
fn test_guest_create_with_name() {
  let mut cmd = run_userkit_command(vec!["guest", "create", "testguest"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account testguest created"));
}

#[test]
fn test_guest_create_with_expiration() {
  let mut cmd = run_userkit_command(vec!["guest", "create", "testguest2", "--expire", "7"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Guest account testguest2 created with 7 day expiration",
  ));
}

#[test]
fn test_guest_remove() {
  let mut cmd = run_userkit_command(vec!["guest", "remove", "testguest"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account testguest removed"));
}

#[test]
fn test_guest_list() {
  let mut cmd = run_userkit_command(vec!["guest", "list"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest accounts"));
}

#[test]
fn test_guest_info() {
  let mut cmd = run_userkit_command(vec!["guest", "info", "testguest2"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account: testguest2"));
}

#[test]
fn test_guest_shell() {
  let mut cmd = run_userkit_command(vec!["guest", "shell"]);

  // This is a special case as it would spawn a shell
  // We're just testing that the command doesn't fail immediately
  cmd.assert().success();
}

#[test]
fn test_guest_expire() {
  let mut cmd = run_userkit_command(vec!["guest", "expire", "testguest2", "14"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Expiration for guest account testguest2 set to 14 days",
  ));
}

#[test]
fn test_guest_remove_nonexistent() {
  let mut cmd = run_userkit_command(vec!["guest", "remove", "nonexistentguest"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_guest_info_nonexistent() {
  let mut cmd = run_userkit_command(vec!["guest", "info", "nonexistentguest"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
