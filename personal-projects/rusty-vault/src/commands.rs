use crate::models::Entry;
use crate::storage::{load_vault, save_vault};

/// Adds a new entry to the vault
#[allow(clippy::missing_errors_doc, clippy::str_to_string)]
pub fn handle_add(site: &str, username: &str, password: &str) -> Result<(), String> {
    let mut vault = load_vault()?;

    let entry = Entry::new(site.to_string(), username.to_string(), password.to_string());
    vault.add_entry(entry)?;

    save_vault(&vault)?;
    println!("✓ Entry added successfully for '{site}'");

    Ok(())
}

/// Lists all entries in the vault
#[allow(clippy::missing_errors_doc)]
pub fn handle_list() -> Result<(), String> {
    let vault = load_vault()?;

    if vault.is_empty() {
        println!("Vault is empty. Use 'add' command to add entries.");
        return Ok(());
    }

    println!("{vault}");
    println!("\nTotal entries: {}", vault.len());

    Ok(())
}

/// Gets a specific entry from the vault
#[allow(clippy::missing_errors_doc)]
pub fn handle_get(site: &str) -> Result<(), String> {
    let vault = load_vault()?;

    vault.find_entry(site).map_or_else(
        || Err(format!("Entry for site '{site}' not found")),
        |entry| {
            println!("Site:     {}", entry.site);
            println!("Username: {}", entry.username);
            println!("Password: {}", entry.password);
            Ok(())
        },
    )
}

/// Deletes an entry from the vault
#[allow(clippy::missing_errors_doc)]
pub fn handle_delete(site: &str) -> Result<(), String> {
    let mut vault = load_vault()?;

    vault.delete_entry(site)?;
    save_vault(&vault)?;

    println!("✓ Entry for '{site}' deleted successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    // These tests would require mocking the file system
    // For now, integration tests can be run manually
}
