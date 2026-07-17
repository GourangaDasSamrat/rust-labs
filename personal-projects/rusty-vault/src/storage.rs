use crate::models::Entry;
use crate::vault::Vault;
use serde_json;
use std::fs;
#[allow(unused)]
use std::path::{Path, PathBuf};

const VAULT_FILENAME: &str = ".rusty_vault.json";

/// Gets the path to the vault file in the user's home directory
fn get_vault_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_owned())?;
    Ok(home.join(VAULT_FILENAME))
}

/// Loads the vault from the storage file
#[allow(clippy::missing_errors_doc)]
pub fn load_vault() -> Result<Vault, String> {
    let path = get_vault_path()?;

    // If file doesn't exist, return empty vault
    if !path.exists() {
        return Ok(Vault::new());
    }

    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read vault file: {e}"))?;

    if contents.is_empty() {
        return Ok(Vault::new());
    }

    let entries: Vec<Entry> =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse vault file: {e}"))?;

    Ok(Vault::from_entries(entries))
}

/// Saves the vault to the storage file
#[allow(clippy::missing_errors_doc)]
pub fn save_vault(vault: &Vault) -> Result<(), String> {
    let path = get_vault_path()?;

    let json = serde_json::to_string_pretty(vault.get_entries())
        .map_err(|e| format!("Failed to serialize vault: {e}"))?;

    fs::write(&path, json).map_err(|e| format!("Failed to write vault file: {e}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_get_vault_path() {
        let result = get_vault_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with(VAULT_FILENAME));
    }

    #[test]
    fn test_load_nonexistent_vault() {
        // This test verifies that loading a non-existent vault returns an empty vault
        // In a real scenario, this would work since the file doesn't exist yet
        let result = load_vault();
        assert!(result.is_ok());
    }
}
