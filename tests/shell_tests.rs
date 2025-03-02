use assert_cmd::prelude::*;
use predicates::prelude::*;
mod test_utils;
use test_utils::run_userkit_command;

#[test]
fn test_shell_profile() {
  let mut cmd = run_userkit_command(vec!["shell", "--profile", "testprofile"]);

  // This is a special case as it would spawn a shell
  // We're just testing that the command doesn't fail immediately
  cmd.assert().success();
}

#[test]
fn test_shell_temp() {
  let mut cmd = run_userkit_command(vec!["shell", "--temp"]);

  // This is a special case as it would spawn a temporary shell
  // We're just testing that the command doesn't fail immediately
  cmd.assert().success();
}

#[test]
fn test_shell_nonexistent_profile() {
  let mut cmd = run_userkit_command(vec!["shell", "--profile", "nonexistentprofile"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
