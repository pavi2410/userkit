use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "userkit")]
#[command(about = "A comprehensive CLI tool for user management across different operating systems", long_about = None)]
#[command(version)]
pub struct Cli {
  #[command(subcommand)]
  pub domain: Domains,
}

#[derive(Subcommand)]
pub enum Domains {
  /// User account management
  #[command(subcommand)]
  User(UserCommands),

  /// Group management
  #[command(subcommand)]
  Group(GroupCommands),

  /// Permission management
  #[command(subcommand)]
  Perm(PermCommands),

  /// Role-based access control
  #[command(subcommand)]
  Role(RoleCommands),

  /// Guest and temporary account management
  #[command(subcommand)]
  Guest(GuestCommands),

  /// Tool configuration
  #[command(subcommand)]
  Config(ConfigCommands),

  /// Shell access and user switching
  #[command(subcommand)]
  Shell(ShellCommands),
}

#[derive(Subcommand)]
pub enum UserCommands {
  /// Create a new user
  Add {
    /// Username for the new user
    username: String,
    /// Home directory for the new user
    #[arg(long)]
    home_dir: Option<String>,
    /// Shell for the new user
    #[arg(long)]
    shell: Option<String>,
    /// User ID for the new user
    #[arg(long)]
    uid: Option<u32>,
    /// Group ID for the new user
    #[arg(long)]
    gid: Option<u32>,
    /// GECOS information for the new user
    #[arg(long)]
    gecos: Option<String>,
  },

  /// Delete a user
  Remove {
    /// Username to remove
    username: String,
    /// Also remove home directory
    #[arg(long)]
    remove_home: bool,
  },

  /// Modify user properties
  Modify {
    /// Username to modify
    username: String,
    /// New home directory
    #[arg(long)]
    home_dir: Option<String>,
    /// New shell
    #[arg(long)]
    shell: Option<String>,
    /// New user ID
    #[arg(long)]
    uid: Option<u32>,
    /// New group ID
    #[arg(long)]
    gid: Option<u32>,
    /// New GECOS information
    #[arg(long)]
    gecos: Option<String>,
  },

  /// List users with filtering options
  List {
    /// Output format
    #[arg(long, short, default_value = "table")]
    format: ListFormat,
    /// Filter by UID range
    #[arg(long)]
    uid_range: Option<String>,
    /// Filter by GID
    #[arg(long)]
    gid: Option<u32>,
  },

  /// Show detailed user information
  Info {
    /// Username to show information for
    username: String,
  },

  /// Lock user account
  Lock {
    /// Username to lock
    username: String,
  },

  /// Unlock user account
  Unlock {
    /// Username to unlock
    username: String,
  },

  /// Change user password
  Passwd {
    /// Username to change password for
    username: String,
  },
}

#[derive(Subcommand)]
pub enum GroupCommands {
  /// Create a new group
  Add {
    /// Group name
    groupname: String,
    /// Group ID
    #[arg(long)]
    gid: Option<u32>,
  },

  /// Delete a group
  Remove {
    /// Group name to remove
    groupname: String,
  },

  /// Modify group properties
  Modify {
    /// Group name to modify
    groupname: String,
    /// New group ID
    #[arg(long)]
    gid: Option<u32>,
  },

  /// List groups with filtering
  List {
    /// Output format
    #[arg(long, short, default_value = "table")]
    format: ListFormat,
  },

  /// List members of a group
  Members {
    /// Group name to list members for
    groupname: String,
  },

  /// Add user to group
  AddUser {
    /// Group name
    groupname: String,
    /// Username to add to group
    username: String,
  },

  /// Remove user from group
  RemoveUser {
    /// Group name
    groupname: String,
    /// Username to remove from group
    username: String,
  },
}

#[derive(Subcommand)]
pub enum PermCommands {
  /// Set permissions
  Set {
    /// Target file or directory
    target: String,
    /// Permissions to set (e.g., "755")
    permissions: String,
    /// Apply recursively
    #[arg(long, short)]
    recursive: bool,
  },

  /// Get permissions for a target
  Get {
    /// Target file or directory
    target: String,
  },

  /// Check if user has permissions
  Check {
    /// Username to check
    username: String,
    /// Target file or directory
    target: String,
  },

  /// Manage sudo access
  Sudo {
    /// Username to manage sudo access for
    username: String,
    /// Enable or disable sudo access
    #[arg(value_enum)]
    action: SudoAction,
  },
}

#[derive(Subcommand)]
pub enum RoleCommands {
  /// Create a new role
  Create {
    /// Role name
    rolename: String,
    /// Description of the role
    #[arg(long)]
    description: Option<String>,
  },

  /// Delete a role
  Delete {
    /// Role name to delete
    rolename: String,
  },

  /// Assign role to user
  Assign {
    /// Role name
    rolename: String,
    /// Username to assign role to
    username: String,
  },

  /// Revoke role from user
  Revoke {
    /// Role name
    rolename: String,
    /// Username to revoke role from
    username: String,
  },

  /// List available roles
  List {
    /// Output format
    #[arg(long, short, default_value = "table")]
    format: ListFormat,
  },

  /// Show role details and permissions
  Info {
    /// Role name
    rolename: String,
  },

  /// Add permission to role
  AddPerm {
    /// Role name
    rolename: String,
    /// Permission to add
    permission: String,
  },
}

#[derive(Subcommand)]
pub enum GuestCommands {
  /// Create a new guest account
  Create {
    /// Guest account name (optional)
    name: Option<String>,
    /// Expiration time in days
    #[arg(long)]
    expire: Option<u32>,
  },

  /// Remove a guest account
  Remove {
    /// Guest account name
    name: String,
  },

  /// List all guest accounts
  List,

  /// Show guest account details
  Info {
    /// Guest account name
    name: String,
  },

  /// Spawn a temporary shell with a one-off guest account
  Shell,

  /// Set expiration for a guest account
  Expire {
    /// Guest account name
    name: String,
    /// Expiration time in days
    time: u32,
  },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
  /// Set configuration option
  Set {
    /// Configuration key
    key: String,
    /// Configuration value
    value: String,
  },

  /// Get configuration option
  Get {
    /// Configuration key
    key: String,
  },

  /// List all configuration options
  List,

  /// Reset configuration to defaults
  Reset,
}

#[derive(Subcommand)]
pub enum ShellCommands {
  /// Switch to a user and start a shell session
  #[command(name = "shell")]
  Shell {
    /// Profile name
    #[arg(long)]
    profile: String,
  },
}

#[derive(ValueEnum, Clone)]
pub enum ListFormat {
  /// Display as a table
  Table,
  /// Display as JSON
  Json,
  /// Display as CSV
  Csv,
}

#[derive(ValueEnum, Clone)]
pub enum SudoAction {
  /// Enable sudo access
  Enable,
  /// Disable sudo access
  Disable,
}
