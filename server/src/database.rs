//! astrobase-server in-memory key-value database.

use anyhow::anyhow;
use std::collections::HashMap;

/// Represents the database internal structure.
#[derive(Default)]
pub struct Database {
    table: HashMap<String, String>,
}

impl Database {
    /// Returns a value or error.
    pub fn get(&self, key: &str) -> anyhow::Result<String> {
        let value = self
            .table
            .get(key)
            .ok_or_else(|| anyhow!("Record '{}' is missing", key))?;
        Ok(value.clone())
    }
}
