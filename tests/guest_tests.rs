use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_guest_create() {
  let mut cmd = Command::new("cargo");
  cmd.arg("run").arg("--").arg("guest").arg("create");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account created"));
}

#[test]
fn test_guest_create_with_name() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("guest")
    .arg("create")
    .arg("testguest");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account testguest created"));
}

#[test]
fn test_guest_create_with_expiration() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("guest")
    .arg("create")
    .arg("testguest2")
    .arg("--expire")
    .arg("7");

  cmd.assert().success().stdout(predicate::str::contains(
    "Guest account testguest2 created with 7 day expiration",
  ));
}

#[test]
fn test_guest_remove() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("guest")
    .arg("remove")
    .arg("testguest");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account testguest removed"));
}

#[test]
fn test_guest_list() {
  let mut cmd = Command::new("cargo");
  cmd.arg("run").arg("--").arg("guest").arg("list");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest accounts"));
}

#[test]
fn test_guest_info() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("guest")
    .arg("info")
    .arg("testguest2");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Guest account: testguest2"));
}

#[test]
fn test_guest_shell() {
  let mut cmd = Command::new("cargo");
  cmd.arg("run").arg("--").arg("guest").arg("shell");

  // This is a special case as it would spawn a shell
  // We're just testing that the command doesn't fail immediately
  cmd.assert().success();
}

#[test]
fn test_guest_expire() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("guest")
    .arg("expire")
    .arg("testguest2")
    .arg("14");

  cmd.assert().success().stdout(predicate::str::contains(
    "Expiration for guest account testguest2 set to 14 days",
  ));
}

#[test]
fn test_guest_remove_nonexistent() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("guest")
    .arg("remove")
    .arg("nonexistentguest");

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_guest_info_nonexistent() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("guest")
    .arg("info")
    .arg("nonexistentguest");

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
