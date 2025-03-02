//! Tests for group management commands

mod common;

use common::{TestEnvironment, start_container, stop_container, run_userkit_command};

#[test]
fn test_group_add_and_remove() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Test adding a group
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["group", "add", "testgroup"]
    );
    assert_eq!(status, 0, "Failed to add group: {}", stderr);
    assert!(stdout.contains("Group testgroup added successfully"));
    
    // Verify group exists
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["group", "list"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("testgroup"));
    
    // Test removing the group
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["group", "remove", "testgroup"]
    );
    assert_eq!(status, 0, "Failed to remove group: {}", stderr);
    assert!(stdout.contains("Group testgroup deleted successfully"));
    
    stop_container(&container_id);
}

#[test]
fn test_group_modify() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Add a test group
    run_userkit_command(
        &container_id,
        &["group", "add", "testgroup"]
    );
    
    // Modify group properties
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["group", "modify", "testgroup", "--gid", "2000"]
    );
    assert_eq!(status, 0, "Failed to modify group: {}", stderr);
    
    // Verify modifications
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["group", "list", "--format", "json"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("\"gid\": 2000"));
    
    stop_container(&container_id);
}

#[test]
fn test_group_members() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Add a test group and user
    run_userkit_command(
        &container_id,
        &["group", "add", "testgroup"]
    );
    
    run_userkit_command(
        &container_id,
        &["user", "add", "testuser"]
    );
    
    // Add user to group
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["group", "adduser", "testgroup", "testuser"]
    );
    assert_eq!(status, 0, "Failed to add user to group: {}", stderr);
    
    // Verify user is in group
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["group", "members", "testgroup"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("testuser"));
    
    // Remove user from group
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["group", "removeuser", "testgroup", "testuser"]
    );
    assert_eq!(status, 0, "Failed to remove user from group: {}", stderr);
    
    // Verify user is no longer in group
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["group", "members", "testgroup"]
    );
    assert_eq!(status, 0);
    assert!(!stdout.contains("testuser"));
    
    stop_container(&container_id);
}

#[test]
fn test_group_list_formats() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Add a test group
    run_userkit_command(
        &container_id,
        &["group", "add", "testgroup"]
    );
    
    // Test listing groups in different formats
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["group", "list", "--format", "table"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("Group Name"));
    assert!(stdout.contains("testgroup"));
    
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["group", "list", "--format", "json"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("\"groupname\": \"testgroup\""));
    
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["group", "list", "--format", "csv"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("testgroup"));
    
    stop_container(&container_id);
}