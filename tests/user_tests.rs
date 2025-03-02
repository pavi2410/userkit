//! Tests for user management commands

mod common;

use common::{TestEnvironment, start_container, stop_container, run_userkit_command};

#[test]
fn test_user_add_and_remove() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Test adding a user
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["user", "add", "testuser", "--home-dir", "/home/testuser"]
    );
    assert_eq!(status, 0, "Failed to add user: {}", stderr);
    assert!(stdout.contains("User testuser added successfully"));
    
    // Verify user exists
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["user", "info", "testuser"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("Username: testuser"));
    assert!(stdout.contains("/home/testuser"));
    
    // Test removing the user
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["user", "remove", "testuser"]
    );
    assert_eq!(status, 0, "Failed to remove user: {}", stderr);
    assert!(stdout.contains("User testuser deleted successfully"));
    
    stop_container(&container_id);
}

#[test]
fn test_user_list() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Add a test user first
    run_userkit_command(
        &container_id,
        &["user", "add", "testuser"]
    );
    
    // Test listing users in different formats
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["user", "list", "--format", "table"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("Username"));
    assert!(stdout.contains("testuser"));
    
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["user", "list", "--format", "json"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("\"username\": \"testuser\""));
    
    stop_container(&container_id);
}

#[test]
fn test_user_modify() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Add a test user
    run_userkit_command(
        &container_id,
        &["user", "add", "testuser"]
    );
    
    // Modify user properties
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["user", "modify", "testuser", "--shell", "/bin/zsh", "--gecos", "Test User"]
    );
    assert_eq!(status, 0, "Failed to modify user: {}", stderr);
    
    // Verify modifications
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["user", "info", "testuser"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("/bin/zsh"));
    assert!(stdout.contains("Test User"));
    
    stop_container(&container_id);
}

#[test]
fn test_user_lock_unlock() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Add a test user
    run_userkit_command(
        &container_id,
        &["user", "add", "testuser"]
    );
    
    // Test locking user
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["user", "lock", "testuser"]
    );
    assert_eq!(status, 0, "Failed to lock user: {}", stderr);
    
    // Test unlocking user
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["user", "unlock", "testuser"]
    );
    assert_eq!(status, 0, "Failed to unlock user: {}", stderr);
    
    stop_container(&container_id);
}