
# Rusty Vault - CLI Password Manager

A secure, lightweight command-line password manager written in Rust. Store, retrieve, and manage your passwords safely on your local machine.

## Features

✓ **Simple CLI Interface** - Easy-to-use commands for managing passwords
✓ **Local Storage** - All passwords stored securely in your home directory
✓ **Duplicate Prevention** - Prevents duplicate site entries
✓ **Error Resilience** - Graceful error handling without crashes
✓ **Clean Display** - Nicely formatted table output

## Installation

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build from Source

```bash
cd rusty-vault
cargo build --release
```

The binary will be available at `target/release/rusty-vault`

## Usage

### Add a new password entry

```bash
./rusty-vault add github.com user@email.com mypassword123
# Output: ✓ Entry added successfully for 'github.com'
```

### List all entries

```bash
./rusty-vault list
# Output: A formatted table with all stored entries
```

### Get password for a specific site

```bash
./rusty-vault get github.com
# Output:
# Site:     github.com
# Username: user@email.com
# Password: mypassword123
```

### Delete an entry

```bash
./rusty-vault delete github.com
# Output: ✓ Entry for 'github.com' deleted successfully
```

### Show help

```bash
./rusty-vault help
```

## Project Structure

```
src/
├── main.rs       - Entry point and CLI argument parsing
├── lib.rs        - Module declarations and public API
├── models.rs     - Entry struct definition
├── vault.rs      - Core Vault logic and data management
├── storage.rs    - File I/O operations (JSON persistence)
└── commands.rs   - Command handlers for all CLI operations
```

## Data Storage

All password entries are stored in a JSON file located at:
- **Linux/Mac**: `~/.rusty_vault.json`
- **Windows**: `%USERPROFILE%\.rusty_vault.json`

The vault file is automatically created on first use.

## Safety Features

### Duplicate Prevention
The `add` command will reject any attempt to add a password for a site that already has an entry:
```bash
./rusty-vault add github.com user1 pass1
./rusty-vault add github.com user2 pass2
# Error: Entry for site 'github.com' already exists
```

### Error Handling
All operations use `Result<T, String>` for safe error handling:
- Missing files are handled gracefully
- Invalid JSON is detected and reported
- Commands validate argument counts
- Site lookups are case-insensitive

### Memory Efficiency
- Uses references (`&str`) for lookups instead of cloning
- Owned `String` data stored only in Entry structs
- Efficient Vec<Entry> storage with no unnecessary allocations

## Examples

### Workflow Example

```bash
# Initialize vault with a few entries
./rusty-vault add github.com john.doe GH_pass123
./rusty-vault add amazon.com john@email.com AWS_pass456
./rusty-vault add mailbox.org john@email.com mail_pass789

# List all entries
./rusty-vault list

# Retrieve a specific password
./rusty-vault get amazon.com

# Update an entry (delete and re-add)
./rusty-vault delete github.com
./rusty-vault add github.com john.doe NEW_password

# Delete when no longer needed
./rusty-vault delete mailbox.org
```

## Architecture

### Entry
Represents a single password entry with:
- `site` - The website/service name
- `username` - Associated username or email
- `password` - The stored password

### Vault
The main container managing all entries:
- Prevents duplicate site entries
- Provides search by site name
- Formats display output as tables
- Supports add, delete, and list operations

### Storage Module
Handles file I/O:
- Loads and saves JSON format
- Manages file paths using user's home directory
- Provides graceful error messages

## Testing

Run the test suite:

```bash
cargo test
```

The project includes unit tests for:
- Entry creation and matching
- Vault operations (add, delete, find)
- Duplicate prevention

## Limitations & Future Improvements

- Passwords are stored in plain text (consider encryption)
- No password strength validator
- No import/export functionality
- No search by username
- Single user per vault

## Security Considerations

⚠️ **Warning**: This is a demonstration project. Passwords are stored in plain text JSON.
For production use, consider:
- Adding encryption (AES-256)
- Using a master password
- Implementing secure memory handling
- Adding audit logging

