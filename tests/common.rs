//! Common test utilities and setup for UserKit tests

use std::process::Command;
use std::time::Duration;

/// Container configuration for different OS environments
pub enum TestEnvironment {
    Ubuntu,
    Alpine,
    // Windows support planned for future
}

impl TestEnvironment {
    /// Get the Docker image name for this environment
    pub fn image_name(&self) -> &'static str {
        match self {
            TestEnvironment::Ubuntu => "ubuntu:latest",
            TestEnvironment::Alpine => "alpine:latest",
        }
    }
    
    /// Get environment-specific setup commands
    pub fn setup_commands(&self) -> Vec<&'static str> {
        match self {
            TestEnvironment::Ubuntu => vec![
                "apt-get update",
                "apt-get install -y sudo"
            ],
            TestEnvironment::Alpine => vec![
                "apk update",
                "apk add sudo"
            ],
        }
    }
}

/// Start a Docker container for testing
pub fn start_container(env: &TestEnvironment) -> String {
    // Start container in detached mode
    let output = Command::new("docker")
        .args([
            "run", 
            "-d", 
            "--rm", 
            env.image_name(),
            "tail", "-f", "/dev/null"
        ])
        .output()
        .expect("Failed to start container");
    
    let container_id = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();
    
    // Run setup commands
    for cmd in env.setup_commands() {
        Command::new("docker")
            .args(["exec", &container_id, "sh", "-c", cmd])
            .output()
            .expect(&format!("Failed to run setup command: {}", cmd));
    }
    
    // Wait a moment for setup to complete
    std::thread::sleep(Duration::from_secs(1));
    
    container_id
}

/// Stop a Docker container
pub fn stop_container(container_id: &str) {
    Command::new("docker")
        .args(["stop", container_id])
        .output()
        .expect("Failed to stop container");
}

/// Run a UserKit command in the container
pub fn run_userkit_command(container_id: &str, args: &[&str]) -> (String, String, i32) {
    // Copy the userkit binary to the container
    Command::new("docker")
        .args(["cp", "../target/debug/userkit", &format!("{container_id}:/usr/local/bin/userkit")])
        .output()
        .expect("Failed to copy userkit binary to container");
    
    // Make it executable
    Command::new("docker")
        .args(["exec", container_id, "chmod", "+x", "/usr/local/bin/userkit"])
        .output()
        .expect("Failed to make userkit executable");
    
    // Run the command
    let output = Command::new("docker")
        .args(
            [&["exec", container_id, "userkit"], args].concat()
        )
        .output()
        .expect("Failed to run userkit command");
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    (stdout, stderr, output.status.code().unwrap_or(-1))
}