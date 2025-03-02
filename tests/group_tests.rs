use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_group_add() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("add")
    .arg("testgroup");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup created"));
}

#[test]
fn test_group_add_with_gid() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("add")
    .arg("testgroup2")
    .arg("--gid")
    .arg("1001");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup2 created"));
}

#[test]
fn test_group_remove() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("remove")
    .arg("testgroup");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup removed"));
}

#[test]
fn test_group_modify() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("modify")
    .arg("testgroup2")
    .arg("--gid")
    .arg("1002");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup2 modified"));
}

#[test]
fn test_group_list() {
  let mut cmd = Command::new("cargo");
  cmd.arg("run").arg("--").arg("group").arg("list");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group name"));
}

#[test]
fn test_group_list_json_format() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("list")
    .arg("--format")
    .arg("json");

  cmd.assert().success().stdout(predicate::str::contains("["));
}

#[test]
fn test_group_members() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("members")
    .arg("testgroup");

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Members of group testgroup"));
}

#[test]
fn test_group_adduser() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("adduser")
    .arg("testgroup")
    .arg("testuser");

  cmd.assert().success().stdout(predicate::str::contains(
    "User testuser added to group testgroup",
  ));
}

#[test]
fn test_group_removeuser() {
  let mut cmd = Command::new("cargo");
  cmd
    .arg("run")
    .arg("--")
    .arg("group")
    .arg("removeuser")
    .arg("testgroup")
    .arg("testuser");

  cmd.assert().success().stdout(predicate::str::contains(
    "User testuser removed from group testgroup",
  ));
}

#[test]
fn test_group_add_invalid() {
  let mut cmd = Command::new("cargo");
  cmd.arg("run").arg("--").arg("group").arg("add").arg("root"); // Trying to add a group that likely already exists

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
