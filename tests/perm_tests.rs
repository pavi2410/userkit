use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_perm_set() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("set")
    .arg("/tmp/testfile")
    .arg("755");

  cmd.assert().success().stdout(predicate::str::contains(
    "Permissions set to 755 for /tmp/testfile",
  ));
}

#[test]
fn test_perm_set_recursive() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("set")
    .arg("/tmp/testdir")
    .arg("755")
    .arg("--recursive");

  cmd.assert().success().stdout(predicate::str::contains(
    "Permissions set recursively to 755 for /tmp/testdir",
  ));
}

#[test]
fn test_perm_get() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("get")
    .arg("/tmp/testfile");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Permissions for /tmp/testfile"));
}

#[test]
fn test_perm_check() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("check")
    .arg("testuser")
    .arg("/tmp/testfile");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("User testuser has"));
}

#[test]
fn test_perm_sudo_enable() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("sudo")
    .arg("testuser")
    .arg("enable");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Sudo access enabled for testuser"));
}

#[test]
fn test_perm_sudo_disable() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("sudo")
    .arg("testuser")
    .arg("disable");

  cmd.assert().success().stdout(predicate::str::contains(
    "Sudo access disabled for testuser",
  ));
}

#[test]
fn test_perm_set_invalid_permissions() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("set")
    .arg("/tmp/testfile")
    .arg("999"); // Invalid permission value

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_perm_check_nonexistent_user() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("perm")
    .arg("check")
    .arg("nonexistentuser")
    .arg("/tmp/testfile");

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
