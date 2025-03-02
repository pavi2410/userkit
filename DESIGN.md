# UserKit: Unified User Management CLI

## Overview

UserKit is a comprehensive command-line interface (CLI) tool built in Rust for simplifying user management across different operating systems. It aims to provide a consistent interface for common user management tasks that are typically scattered across various commands with different syntaxes.

## Design Philosophy

1. **Consistency**: Uniform command structure across all operations

2. **Simplicity**: Easy-to-remember commands that abstract complex operations

3. **Cross-platform**: Support for Linux, macOS, and Windows (planned)

4. **Extensibility**: Modular design to support future features

5. **Native Implementation**: Use Rust's native APIs where possible instead of wrapping existing commands

## Command Structure

UserKit follows a consistent command structure:

```
userkit <domain> <action> [options]
```

### Domains

- `user`: User account management
- `group`: Group management
- `perm`: Permission management
- `role`: Role-based access control
- `guest`: Guest and temporary account management
- `config`: Tool configuration

## Feature Set

### 1. User Management

| Command | Description |
|---------|-------------|
| `userkit user add <username> [options]` | Create a new user |
| `userkit user remove <username> [options]` | Delete a user |
| `userkit user modify <username> [options]` | Modify user properties |
| `userkit user list [options]` | List users with filtering options |
| `userkit user info <username>` | Show detailed user information |
| `userkit user lock <username>` | Lock user account |
| `userkit user unlock <username>` | Unlock user account |
| `userkit user passwd <username>` | Change user password |
| `userkit user shell --username <username> <command>` | Switch to a user and start a shell session or run a command |
| `userkit user shell --temp` | Create a temporary user and start a shell session |

### 2. Group Management

| Command | Description |
|---------|-------------|
| `userkit group add <groupname> [options]` | Create a new group |
| `userkit group remove <groupname>` | Delete a group |
| `userkit group modify <groupname> [options]` | Modify group properties |
| `userkit group list [options]` | List groups with filtering |
| `userkit group members <groupname>` | List members of a group |
| `userkit group adduser <groupname> <username>` | Add user to group |
| `userkit group removeuser <groupname> <username>` | Remove user from group |

### 3. Permission Management

| Command | Description |
|---------|-------------|
| `userkit perm set <target> <permissions> [options]` | Set permissions |
| `userkit perm get <target>` | Get permissions for a target |
| `userkit perm check <username> <target>` | Check if user has permissions |
| `userkit perm sudo <username> [enable/disable]` | Manage sudo access |

### 4. Role-Based Access Control

| Command | Description |
|---------|-------------|
| `userkit role create <rolename> [options]` | Create a new role |
| `userkit role delete <rolename>` | Delete a role |
| `userkit role assign <rolename> <username>` | Assign role to user |
| `userkit role revoke <rolename> <username>` | Revoke role from user |
| `userkit role list [options]` | List available roles |
| `userkit role info <rolename>` | Show role details and permissions |
| `userkit role addperm <rolename> <permission>` | Add permission to role |

### 5. Guest Account Management

| Command | Description |
|---------|-------------|
| `userkit guest create [name] [options]` | Create a new guest account |
| `userkit guest remove <name>` | Remove a guest account |
| `userkit guest list` | List all guest accounts |
| `userkit guest info <name>` | Show guest account details |
| `userkit guest shell [options]` | Spawn a temporary shell with a one-off guest account |
| `userkit guest expire <name> <time>` | Set expiration for a guest account |

## Technical Implementation

### Core Components

1. **Command Parser**: Process and validate user input

2. **Platform Abstraction Layer**: Abstract OS-specific operations

3. **User Management Core**: Implement user operations

4. **Group Management Core**: Implement group operations

5. **Permission System**: Handle file and resource permissions

6. **Role Manager**: Implement role-based access control

7. **Guest Account Manager**: Handle temporary and guest accounts

8. **Configuration Manager**: Handle tool settings and defaults

### Platform-Specific Implementations

- **Linux**: Use native Linux user management APIs and files

- **macOS**: Implement Directory Services and OpenDirectory integration

- **Windows**: Use Windows Management Instrumentation (WMI) and Active Directory

## Development Roadmap

### Phase 1: Core Functionality (Linux)

1. Basic CLI structure and command parsing

2. User management operations (add, remove, modify, list, info)

3. Group management operations (add, remove, modify, list, members)

4. Basic permission management (file permissions, sudo access)

### Phase 2: Enhanced Features (Linux)

1. Role-based access control

2. Guest account management

3. Temporary shell sessions

4. Configuration system

### Phase 3: Cross-Platform Support

1. macOS support

2. Windows support

3. Platform detection and abstraction refinement

### Phase 4: Advanced Features

1. Remote user management (SSH)

2. Directory service integration (LDAP, Active Directory)

3. Batch operations and scripting support

4. Advanced permission management

### Phase 5: Enterprise Features

1. Audit logging

2. Compliance reporting

3. Enterprise authentication integration

4. Advanced security features

## Configuration

UserKit will use a TOML configuration file located at:

- Linux, macOS: `/etc/userkit/config.toml`

- Windows: `%ProgramData%\userkit\config.toml`