# UserKit

UserKit is a comprehensive command-line interface (CLI) tool built in Rust for simplifying user management across different operating systems. It provides a consistent interface for common user management tasks that are typically scattered across various commands with different syntaxes.

## Features

- User account management (add, remove, modify, list, info)
- Interactive shell sessions
- Multiple output formats (Table, JSON, CSV)
- Cross-platform compatibility (in development)

## Installation

### From Source

1. Ensure you have Rust and Cargo installed
2. Clone the repository:
   ```
   git clone https://github.com/pavi2410/userkit.git
   cd userkit
   ```
3. Build and install:
   ```
   cargo install --path .
   ```

## Usage

UserKit follows a consistent command structure:

```
userkit <domain> <action> [options]
```

### Examples

1. List all users in table format:
   ```
   userkit user list --format table
   ```

2. Add a new user:
   ```
   userkit user add username --home-dir /home/username
   ```

3. Get detailed information about a user:
   ```
   userkit user info username
   ```

4. Start a shell session:
   ```
   userkit user shell --username username
   ```

5. Execute a command as another user:
   ```
   userkit user shell --username username "command"
   ```

## Development

### Prerequisites

- Rust (latest stable)
- Cargo

### Setup

1. Clone the repository
2. Install dependencies:
   ```
   cargo build
   ```
3. Run tests:
   ```
   cargo test
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## References

- [Linux User Management Guide](https://phoenixnap.com/kb/user-management-linux)