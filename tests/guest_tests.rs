//! Tests for guest account management commands

mod common;

use common::{TestEnvironment, start_container, stop_container, run_userkit_command};

#[test]
fn test_guest_create_and_remove() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Test creating a guest account
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["guest", "create", "testguest", "--expire", "7"]
    );
    assert_eq!(status, 0, "Failed to create guest account: {}", stderr);
    assert!(stdout.contains("Guest account testguest created successfully"));
    
    // Verify guest account exists
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["guest", "info", "testguest"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("Name: testguest"));
    assert!(stdout.contains("Expires in: 7 days"));
    
    // Test removing the guest account
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["guest", "remove", "testguest"]
    );
    assert_eq!(status, 0, "Failed to remove guest account: {}", stderr);
    assert!(stdout.contains("Guest account testguest removed successfully"));
    
    stop_container(&container_id);
}

#[test]
fn test_guest_list() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create multiple guest accounts
    run_userkit_command(
        &container_id,
        &["guest", "create", "guest1", "--expire", "3"]
    );
    
    run_userkit_command(
        &container_id,
        &["guest", "create", "guest2", "--expire", "5"]
    );
    
    // Test listing guest accounts
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["guest", "list"]
    );
    assert_eq!(status, 0, "Failed to list guest accounts: {}", stderr);
    assert!(stdout.contains("guest1"));
    assert!(stdout.contains("guest2"));
    
    stop_container(&container_id);
}

#[test]
fn test_guest_expire() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Create a guest account
    run_userkit_command(
        &container_id,
        &["guest", "create", "testguest", "--expire", "3"]
    );
    
    // Test changing expiration
    let (stdout, stderr, status) = run_userkit_command(
        &container_id,
        &["guest", "expire", "testguest", "10"]
    );
    assert_eq!(status, 0, "Failed to change expiration: {}", stderr);
    assert!(stdout.contains("Expiration for testguest set to 10 days"));
    
    // Verify expiration was changed
    let (stdout, _, status) = run_userkit_command(
        &container_id,
        &["guest", "info", "testguest"]
    );
    assert_eq!(status, 0);
    assert!(stdout.contains("Expires in: 10 days"));
    
    stop_container(&container_id);
}

#[test]
fn test_guest_shell() {
    let container_id = start_container(&TestEnvironment::Ubuntu);
    
    // Test spawning a temporary shell
    // This is a bit tricky to test in an automated way, so we'll just check if the command runs
    let (_, stderr, status) = run_userkit_command(
        &container_id,
        &["guest", "shell"]
    );
    assert_eq!(status, 0, "Failed to spawn guest shell: {}", stderr);
    
    stop_container(&container_id);
}