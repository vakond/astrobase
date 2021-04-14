//! astrobase-server in-memory key-value database.

use anyhow::anyhow;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Represents the database internal structure.
pub struct Database {
    table: RwLock<HashMap<String, String>>,
}

impl Database {
    /// Construct new instance of the database.
    pub fn new() -> Self {
        Database {
            table: RwLock::new(HashMap::new()),
        }
    }

    /// Returns a value or error.
    pub async fn get(&self, key: &str) -> anyhow::Result<String> {
        let table = self.table.read().await;
        let value = table
            .get(key)
            .ok_or_else(|| anyhow!("Record '{}' is missing", key))?;
        Ok(value.clone())
    }

    /// Inserts new record if there was no such key or returns error.
    pub async fn insert(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let mut table = self.table.write().await;
        if table.contains_key(key) {
            return Err(anyhow!("Record '{}' already exists", key));
        }
        table.insert(key.into(), value.into());
        Ok("".into())
    }

    /// Deletes a record or returns error if was missing.
    pub async fn delete(&self, key: &str) -> anyhow::Result<String> {
        let mut table = self.table.write().await;
        let value = table
            .remove(key)
            .ok_or_else(|| anyhow!("Record '{}' is missing already", key))?;
        Ok(value.clone())
    }

    /// Updates record or returns error if the record was missing or identical.
    pub async fn update(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let mut table = self.table.write().await;
        let old_value = table
            .get(key)
            .ok_or_else(|| anyhow!("Record '{}' is missing", key))?;
        if value == old_value {
            return Err(anyhow!("Record '{}' already exists and identical", key));
        }
        *table.get_mut(key).unwrap() = value.into();
        Ok("".into())
    }
}
