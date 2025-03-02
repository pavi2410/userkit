use assert_cmd::prelude::*;
use predicates::prelude::*;
mod test_utils;
use test_utils::run_userkit_command;

#[test]
fn test_role_create() {
  let mut cmd = run_userkit_command(vec!["role", "create", "testrole"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Role testrole created"));
}

#[test]
fn test_role_create_with_description() {
  let mut cmd = run_userkit_command(vec![
    "role",
    "create",
    "testrole2",
    "--description",
    "A test role with description",
  ]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Role testrole2 created"));
}

#[test]
fn test_role_delete() {
  let mut cmd = run_userkit_command(vec!["role", "delete", "testrole"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Role testrole deleted"));
}

#[test]
fn test_role_assign() {
  let mut cmd = run_userkit_command(vec!["role", "assign", "testrole2", "testuser"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Role testrole2 assigned to user testuser",
  ));
}

#[test]
fn test_role_revoke() {
  let mut cmd = run_userkit_command(vec!["role", "revoke", "testrole2", "testuser"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Role testrole2 revoked from user testuser",
  ));
}

#[test]
fn test_role_list() {
  let mut cmd = run_userkit_command(vec!["role", "list"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Available roles"));
}

#[test]
fn test_role_list_json_format() {
  let mut cmd = run_userkit_command(vec!["role", "list", "--format", "json"]);

  cmd.assert().success().stdout(predicate::str::contains("["));
}

#[test]
fn test_role_info() {
  let mut cmd = run_userkit_command(vec!["role", "info", "testrole2"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Role: testrole2"));
}

#[test]
fn test_role_addperm() {
  let mut cmd = run_userkit_command(vec!["role", "addperm", "testrole2", "read:/tmp/testfile"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Permission read:/tmp/testfile added to role testrole2",
  ));
}

#[test]
fn test_role_delete_nonexistent() {
  let mut cmd = run_userkit_command(vec!["role", "delete", "nonexistentrole"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_role_assign_nonexistent_role() {
  let mut cmd = run_userkit_command(vec!["role", "assign", "nonexistentrole", "testuser"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_role_assign_nonexistent_user() {
  let mut cmd = run_userkit_command(vec!["role", "assign", "testrole2", "nonexistentuser"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
