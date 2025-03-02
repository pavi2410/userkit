use assert_cmd::prelude::*;
use predicates::prelude::*;
mod test_utils;
use test_utils::run_userkit_command;

#[test]
fn test_perm_set() {
  let mut cmd = run_userkit_command(vec!["perm", "set", "/tmp/testfile", "755"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Permissions set to 755 for /tmp/testfile",
  ));
}

#[test]
fn test_perm_set_recursive() {
  let mut cmd = run_userkit_command(vec!["perm", "set", "/tmp/testdir", "755", "--recursive"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Permissions set recursively to 755 for /tmp/testdir",
  ));
}

#[test]
fn test_perm_get() {
  let mut cmd = run_userkit_command(vec!["perm", "get", "/tmp/testfile"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Permissions for /tmp/testfile"));
}

#[test]
fn test_perm_check() {
  let mut cmd = run_userkit_command(vec!["perm", "check", "testuser", "/tmp/testfile"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser has"));
}

#[test]
fn test_perm_sudo_enable() {
  let mut cmd = run_userkit_command(vec!["perm", "sudo", "testuser", "enable"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Sudo access enabled for testuser"));
}

#[test]
fn test_perm_sudo_disable() {
  let mut cmd = run_userkit_command(vec!["perm", "sudo", "testuser", "disable"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Sudo access disabled for testuser",
  ));
}

#[test]
fn test_perm_set_invalid_permissions() {
  let mut cmd = run_userkit_command(vec!["perm", "set", "/tmp/testfile", "999"]); // Invalid permission value

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_perm_check_nonexistent_user() {
  let mut cmd = run_userkit_command(vec!["perm", "check", "nonexistentuser", "/tmp/testfile"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
