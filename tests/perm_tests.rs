//! Tests for permission management commands

mod common;
use std::process::Command;
use common::{TestEnvironment, start_container, stop_container, run_userkit_command};

#[test]
fn test_perm_set_and_get() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a test file
    Command::new("docker")
        .args(["exec", &container_id, "sh", "-c", "touch /tmp/testfile"])
        .output()
        .expect("Failed to create test file");
    
    // Test setting permissions
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["perm", "set", "/tmp/testfile", "755"]
    );
    assert_eq!(status, 0, "Failed to set permissions: {}", stderr);
    assert!(stdout.contains("Permissions set successfully"));
    
    // Test getting permissions
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["perm", "get", "/tmp/testfile"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("755"));
    
    stop_container(&container_id);
}

#[test]
fn test_perm_recursive() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a test directory with files
    Command::new("docker")
        .args(["exec", &container_id, "sh", "-c", "mkdir -p /tmp/testdir && touch /tmp/testdir/file1 /tmp/testdir/file2"])
        .output()
        .expect("Failed to create test directory");
    
    // Test setting permissions recursively
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["perm", "set", "/tmp/testdir", "755", "--recursive"]
    );
    assert_eq!(status, 0, "Failed to set recursive permissions: {}", stderr);
    
    // Verify permissions on subdirectory files
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["perm", "get", "/tmp/testdir/file1"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("755"));
    
    stop_container(&container_id);
}

#[test]
fn test_perm_check() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a test user and file
    run_userkit_command(
        &container_id,
        &["user", "add", "testuser"]
    );
    
    Command::new("docker")
        .args(["exec", &container_id, "sh", "-c", "touch /tmp/testfile && chmod 644 /tmp/testfile"])
        .output()
        .expect("Failed to create test file");
    
    // Test checking permissions
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["perm", "check", "testuser", "/tmp/testfile"]
    );
    assert_eq!(status, 0, "Failed to check permissions: {}", stderr);
    assert!(stdout.contains("read"));
    
    stop_container(&container_id);
}

#[test]
fn test_sudo_access() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a test user
    run_userkit_command(
        &container_id,
        &["user", "add", "testuser"]
    );
    
    // Test enabling sudo access
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["perm", "sudo", "testuser", "enable"]
    );
    assert_eq!(status, 0, "Failed to enable sudo access: {}", stderr);
    assert!(stdout.contains("Sudo access enabled for testuser"));
    
    // Test disabling sudo access
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["perm", "sudo", "testuser", "disable"]
    );
    assert_eq!(status, 0, "Failed to disable sudo access: {}", stderr);
    assert!(stdout.contains("Sudo access disabled for testuser"));
    
    stop_container(&container_id);
}