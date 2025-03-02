use assert_cmd::prelude::*;
use predicates::prelude::*;
mod test_utils;
use test_utils::run_userkit_command;

#[test]
fn test_group_new() {
  let mut cmd = run_userkit_command(vec!["group", "new", "testgroup"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup created"));
}

#[test]
fn test_group_new_with_gid() {
  let mut cmd = run_userkit_command(vec!["group", "new", "testgroup2", "--gid", "1001"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup2 created"));
}

#[test]
fn test_group_remove() {
  let mut cmd = run_userkit_command(vec!["group", "remove", "testgroup"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup removed"));
}

#[test]
fn test_group_modify() {
  let mut cmd = run_userkit_command(vec!["group", "modify", "testgroup2", "--gid", "1002"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group testgroup2 modified"));
}

#[test]
fn test_group_list() {
  let mut cmd = run_userkit_command(vec!["group", "list"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Group name"));
}

#[test]
fn test_group_list_json_format() {
  let mut cmd = run_userkit_command(vec!["group", "list", "--format", "json"]);

  cmd.assert().success().stdout(predicate::str::contains("["));
}

#[test]
fn test_group_members() {
  let mut cmd = run_userkit_command(vec!["group", "members", "testgroup"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Members of group testgroup"));
}

#[test]
fn test_group_adduser() {
  let mut cmd = run_userkit_command(vec!["group", "add-user", "testgroup", "testuser"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "User testuser added to group testgroup",
  ));
}

#[test]
fn test_group_removeuser() {
  let mut cmd = run_userkit_command(vec!["group", "remove-user", "testgroup", "testuser"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "User testuser removed from group testgroup",
  ));
}

#[test]
fn test_group_new_invalid() {
  let mut cmd = run_userkit_command(vec!["group", "new", "root"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
