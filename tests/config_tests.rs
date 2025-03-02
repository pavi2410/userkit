use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_config_set() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("config")
        .arg("set")
        .arg("default.shell")
        .arg("/bin/bash");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Configuration option default.shell set to /bin/bash"));
}

#[test]
fn test_config_get() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("config")
        .arg("get")
        .arg("default.shell");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("default.shell"));
}

#[test]
fn test_config_list() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("config")
        .arg("list");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Configuration options"));
}

#[test]
fn test_config_reset() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("config")
        .arg("reset");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Configuration reset to defaults"));
}

#[test]
fn test_config_get_nonexistent() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("config")
        .arg("get")
        .arg("nonexistent.option");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}