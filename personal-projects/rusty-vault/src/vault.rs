use crate::models::Entry;
use std::fmt;

/// The main vault structure that manages password entries
#[derive(Debug, Clone)]
pub struct Vault {
    entries: Vec<Entry>,
}

impl Vault {
    /// Creates a new empty vault
    #[allow(clippy::must_use_candidate)]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates a vault from existing entries
    #[allow(clippy::must_use_candidate)]
    pub const fn from_entries(entries: Vec<Entry>) -> Self {
        Self { entries }
    }

    /// Adds a new entry to the vault
    /// Returns an error if an entry with the same site already exists
    #[allow(clippy::missing_errors_doc)]
    pub fn add_entry(&mut self, entry: Entry) -> Result<(), String> {
        if self.find_entry(&entry.site).is_some() {
            return Err(format!("Entry for site '{}' already exists", entry.site));
        }
        self.entries.push(entry);
        Ok(())
    }

    /// Finds an entry by site name
    #[allow(clippy::must_use_candidate)]
    pub fn find_entry(&self, site: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.matches_site(site))
    }

    /// Finds and returns a mutable reference to an entry
    #[allow(unused)]
    fn find_entry_mut(&mut self, site: &str) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|e| e.matches_site(site))
    }

    /// Deletes an entry by site name
    #[allow(clippy::missing_errors_doc)]
    pub fn delete_entry(&mut self, site: &str) -> Result<(), String> {
        let original_len = self.entries.len();
        self.entries.retain(|e| !e.matches_site(site));

        if self.entries.len() < original_len {
            Ok(())
        } else {
            Err(format!("Entry for site '{site}' not found"))
        }
    }

    /// Returns all entries
    #[allow(clippy::must_use_candidate)]
    pub fn get_entries(&self) -> &[Entry] {
        &self.entries
    }

    /// Returns the number of entries in the vault
    #[allow(clippy::must_use_candidate)]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks if the vault is empty
    #[allow(clippy::must_use_candidate)]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Vault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.entries.is_empty() {
            write!(f, "Vault is empty")?;
            return Ok(());
        }

        // Calculate column widths
        let site_width = self
            .entries
            .iter()
            .map(|e| e.site.len())
            .max()
            .unwrap_or(4)
            .max(4); // "Site" is 4 chars

        let username_width = self
            .entries
            .iter()
            .map(|e| e.username.len())
            .max()
            .unwrap_or(8)
            .max(8); // "Username" is 8 chars

        let password_width = 8; // "Password" is 8 chars

        // Print header
        writeln!(
            f,
            "{:<width1$} | {:<width2$} | {:<width3$}",
            "Site",
            "Username",
            "Password",
            width1 = site_width,
            width2 = username_width,
            width3 = password_width
        )?;

        // Print separator
        let separator = format!(
            "{}-+-{}-+-{}",
            "-".repeat(site_width),
            "-".repeat(username_width),
            "-".repeat(password_width)
        );
        writeln!(f, "{separator}")?;

        // Print entries
        for entry in &self.entries {
            writeln!(
                f,
                "{:<width1$} | {:<width2$} | {:<width3$}",
                entry.site,
                entry.username,
                entry.password,
                width1 = site_width,
                width2 = username_width,
                width3 = password_width
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_creation() {
        let vault = Vault::new();
        assert!(vault.is_empty());
        assert_eq!(vault.len(), 0);
    }

    #[test]
    #[allow(clippy::str_to_string)]
    fn test_add_entry() {
        let mut vault = Vault::new();
        let entry = Entry::new(
            "github.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        assert!(vault.add_entry(entry).is_ok());
        assert_eq!(vault.len(), 1);
    }

    #[test]
    #[allow(clippy::str_to_string, clippy::unwrap_used)]
    fn test_duplicate_prevention() {
        let mut vault = Vault::new();
        let entry1 = Entry::new(
            "github.com".to_string(),
            "user1".to_string(),
            "pass1".to_string(),
        );
        let entry2 = Entry::new(
            "github.com".to_string(),
            "user2".to_string(),
            "pass2".to_string(),
        );

        vault.add_entry(entry1).unwrap();
        let result = vault.add_entry(entry2);
        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::str_to_string, clippy::unwrap_used)]
    fn test_find_entry() {
        let mut vault = Vault::new();
        let entry = Entry::new(
            "github.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        vault.add_entry(entry.clone()).unwrap();

        let found = vault.find_entry("github.com");
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &entry);
    }

    #[test]
    #[allow(clippy::str_to_string, clippy::unwrap_used)]
    fn test_delete_entry() {
        let mut vault = Vault::new();
        let entry = Entry::new(
            "github.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        vault.add_entry(entry).unwrap();

        assert!(vault.delete_entry("github.com").is_ok());
        assert!(vault.is_empty());
    }

    #[test]
    fn test_delete_nonexistent_entry() {
        let mut vault = Vault::new();
        let result = vault.delete_entry("github.com");
        assert!(result.is_err());
    }
}
