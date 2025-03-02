use assert_cmd::prelude::*;
use predicates::prelude::*;
mod test_utils;
use test_utils::run_userkit_command;

#[test]
fn test_config_set() {
  let mut cmd = run_userkit_command(vec!["config", "set", "default.shell", "/bin/bash"]);

  cmd.assert().success().stdout(predicate::str::contains(
    "Configuration option default.shell set to /bin/bash",
  ));
}

#[test]
fn test_config_get() {
  let mut cmd = run_userkit_command(vec!["config", "get", "default.shell"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("default.shell"));
}

#[test]
fn test_config_list() {
  let mut cmd = run_userkit_command(vec!["config", "list"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Configuration options"));
}

#[test]
fn test_config_reset() {
  let mut cmd = run_userkit_command(vec!["config", "reset"]);

  cmd
    .assert()
    .success()
    .stdout(predicate::str::contains("Configuration reset to defaults"));
}

#[test]
fn test_config_get_nonexistent() {
  let mut cmd = run_userkit_command(vec!["config", "get", "nonexistent.option"]);

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("Error"));
}
