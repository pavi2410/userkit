use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_user_add() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("user").arg("add").arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User testuser created"));
}

#[test]
fn test_user_add_with_options() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("add")
        .arg("testuser2")
        .arg("--home-dir")
        .arg("/home/testuser2")
        .arg("--shell")
        .arg("/bin/bash");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User testuser2 created"));
}

#[test]
fn test_user_list() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("user").arg("list");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Username"));
}

#[test]
fn test_user_list_json_format() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("list")
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("["));
}

#[test]
fn test_user_info() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("info")
        .arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Username: testuser"));
}

#[test]
fn test_user_remove() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("remove")
        .arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User testuser removed"));
}

#[test]
fn test_user_remove_with_home() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("remove")
        .arg("testuser2")
        .arg("--remove-home");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User testuser2 removed"));
}

#[test]
fn test_user_modify() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("modify")
        .arg("testuser")
        .arg("--shell")
        .arg("/bin/zsh");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User testuser modified"));
}

#[test]
fn test_user_lock() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("lock")
        .arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User testuser locked"));
}

#[test]
fn test_user_unlock() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("unlock")
        .arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("User testuser unlocked"));
}

#[test]
fn test_user_passwd() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("passwd")
        .arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Password for testuser changed"));
}

#[test]
fn test_user_add_invalid() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("user")
        .arg("add")
        .arg("root"); // Trying to add a user that likely already exists
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}