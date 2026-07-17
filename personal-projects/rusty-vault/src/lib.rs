/// Core data models for the password manager
pub mod models;

/// Vault management logic
pub mod vault;

/// File storage operations
pub mod storage;

/// Command handlers for CLI operations
pub mod commands;

pub use commands::{handle_add, handle_delete, handle_get, handle_list};
pub use models::Entry;
pub use vault::Vault;
