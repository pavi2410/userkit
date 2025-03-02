use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_role_create() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("create")
        .arg("testrole");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Role testrole created"));
}

#[test]
fn test_role_create_with_description() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("create")
        .arg("testrole2")
        .arg("--description")
        .arg("A test role with description");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Role testrole2 created"));
}

#[test]
fn test_role_delete() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("delete")
        .arg("testrole");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Role testrole deleted"));
}

#[test]
fn test_role_assign() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("assign")
        .arg("testrole2")
        .arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Role testrole2 assigned to user testuser"));
}

#[test]
fn test_role_revoke() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("revoke")
        .arg("testrole2")
        .arg("testuser");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Role testrole2 revoked from user testuser"));
}

#[test]
fn test_role_list() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("list");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available roles"));
}

#[test]
fn test_role_list_json_format() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("list")
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("["));
}

#[test]
fn test_role_info() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("info")
        .arg("testrole2");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Role: testrole2"));
}

#[test]
fn test_role_addperm() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("addperm")
        .arg("testrole2")
        .arg("read:/tmp/testfile");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Permission read:/tmp/testfile added to role testrole2"));
}

#[test]
fn test_role_delete_nonexistent() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("delete")
        .arg("nonexistentrole");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_role_assign_nonexistent_role() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("assign")
        .arg("nonexistentrole")
        .arg("testuser");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_role_assign_nonexistent_user() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("role")
        .arg("assign")
        .arg("testrole2")
        .arg("nonexistentuser");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}