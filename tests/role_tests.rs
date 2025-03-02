//! Tests for role-based access control commands

mod common;

use common::{TestEnvironment, start_container, stop_container, run_userkit_command};

#[test]
fn test_role_create_and_delete() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Test creating a role
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["role", "create", "admin", "--description", "Administrator role"]
    );
    assert_eq!(status, 0, "Failed to create role: {}", stderr);
    assert!(stdout.contains("Role admin created successfully"));
    
    // Verify role exists
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["role", "list"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("admin"));
    
    // Test deleting the role
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["role", "delete", "admin"]
    );
    assert_eq!(status, 0, "Failed to delete role: {}", stderr);
    assert!(stdout.contains("Role admin deleted successfully"));
    
    stop_container(&container_id);
}

#[test]
fn test_role_assign_and_revoke() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a test user and role
    run_userkit_command(
        &container_id,
        &["user", "add", "testuser"]
    );
    
    run_userkit_command(
        &container_id,
        &["role", "create", "testrole"]
    );
    
    // Test assigning role to user
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["role", "assign", "testrole", "testuser"]
    );
    assert_eq!(status, 0, "Failed to assign role: {}", stderr);
    assert!(stdout.contains("Role testrole assigned to testuser"));
    
    // Verify role assignment
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["role", "info", "testrole"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("testuser"));
    
    // Test revoking role from user
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["role", "revoke", "testrole", "testuser"]
    );
    assert_eq!(status, 0, "Failed to revoke role: {}", stderr);
    assert!(stdout.contains("Role testrole revoked from testuser"));
    
    stop_container(&container_id);
}

#[test]
fn test_role_add_permission() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a test role
    run_userkit_command(
        &container_id,
        &["role", "create", "testrole"]
    );
    
    // Test adding permission to role
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["role", "addperm", "testrole", "read:/etc"]
    );
    assert_eq!(status, 0, "Failed to add permission: {}", stderr);
    assert!(stdout.contains("Permission added to testrole"));
    
    // Verify permission was added
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["role", "info", "testrole"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("read:/etc"));
    
    stop_container(&container_id);
}

#[test]
fn test_role_list_formats() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a test role
    run_userkit_command(
        &container_id,
        &["role", "create", "testrole", "--description", "Test role"]
    );
    
    // Test listing roles in different formats
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["role", "list", "--format", "table"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("Role Name"));
    assert!(stdout.contains("testrole"));
    
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["role", "list", "--format", "json"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("\"rolename\": \"testrole\""));
    
    stop_container(&container_id);
}