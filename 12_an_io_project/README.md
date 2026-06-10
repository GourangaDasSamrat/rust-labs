# minigrep

A command-line search tool written in Rust that searches for a query string within a file and displays matching lines.

## What is this?

This project is a simple implementation of a grep-like search utility. It supports both case-sensitive and case-insensitive searches.

## Building

```bash
cargo build --release
```

## Running

### Basic usage (case-sensitive):
```bash
cargo run -- <query> <file_path>
```

Example:
```bash
cargo run -- "logic" letter.txt
```

### Case-insensitive search:
Set the `IGNORE_CASE` environment variable:
```bash
IGNORE_CASE=1 cargo run -- <query> <file_path>
```

Example:
```bash
IGNORE_CASE=1 cargo run -- "rust" letter.txt
```

## Testing

Run the test suite:
```bash
cargo test
```

## Features

- Case-sensitive search (default)
- Case-insensitive search (via `IGNORE_CASE` environment variable)
- Displays all lines containing the query
- Error handling for missing files or arguments
