use serde::{Deserialize, Serialize};

/// Represents a single password entry in the vault
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub site: String,
    pub username: String,
    pub password: String,
}

impl Entry {
    /// Creates a new Entry
    #[allow(clippy::must_use_candidate)]
    pub const fn new(site: String, username: String, password: String) -> Self {
        Self {
            site,
            username,
            password,
        }
    }

    /// Checks if the site matches the given name (case-insensitive)
    #[allow(clippy::must_use_candidate)]
    pub fn matches_site(&self, site: &str) -> bool {
        self.site.to_lowercase() == site.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::str_to_string)]
    fn test_entry_creation() {
        let entry = Entry::new(
            "github.com".to_string(),
            "user@email.com".to_string(),
            "secret123".to_string(),
        );
        assert_eq!(entry.site, "github.com");
        assert_eq!(entry.username, "user@email.com");
        assert_eq!(entry.password, "secret123");
    }

    #[test]
    #[allow(clippy::str_to_string)]
    fn test_matches_site_case_insensitive() {
        let entry = Entry::new(
            "GitHub.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        assert!(entry.matches_site("github.com"));
        assert!(entry.matches_site("GITHUB.COM"));
        assert!(entry.matches_site("GitHub.com"));
    }
}
