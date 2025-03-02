use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_shell_profile() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("shell")
    .arg("--profile")
    .arg("testprofile");

  // This is a special case as it would spawn a shell
  // We're just testing that the command doesn't fail immediately
  cmd.assert().success();
}

#[test]
fn test_shell_temp() {
  let mut cmd = Command::new("cargo");
  cmd.arg("run").arg("--").arg("shell").arg("--temp");

  // This is a special case as it would spawn a temporary shell
  // We're just testing that the command doesn't fail immediately
  cmd.assert().success();
}

#[test]
fn test_shell_nonexistent_profile() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("shell")
    .arg("--profile")
    .arg("nonexistentprofile");

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
