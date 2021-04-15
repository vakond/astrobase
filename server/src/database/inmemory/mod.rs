//! astrobase-server in-memory key-value database.

#[cfg(test)]
mod test;

use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Represents the database internal structure.
pub struct InMemory {
    table: RwLock<HashMap<String, String>>,
}

#[async_trait]
impl super::Database for InMemory {
    /// Construct new instance of the database.
    fn new() -> Self {
        InMemory {
            table: RwLock::new(HashMap::new()),
        }
    }

    /// Returns a value or error.
    async fn get(&self, key: &str) -> anyhow::Result<String> {
        let table = self.table.read().await;
        let value = table
            .get(key)
            .ok_or_else(|| anyhow!("Record '{}' is missing", key))?;
        Ok(value.clone())
    }

    /// Inserts new record if there was no such key or returns error.
    async fn insert(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let mut table = self.table.write().await;
        match table.entry(key.into()) {
            Occupied(_) => return Err(anyhow!("Record '{}' already exists", key)),
            Vacant(entry) => entry.insert(value.into()),
        };
        Ok(String::default())
    }

    /// Deletes a record or returns error if was missing.
    async fn delete(&self, key: &str) -> anyhow::Result<String> {
        let mut table = self.table.write().await;
        let value = table
            .remove(key)
            .ok_or_else(|| anyhow!("Record '{}' is missing already", key))?;
        Ok(value.clone())
    }

    /// Updates record or returns error if the record was missing or identical.
    async fn update(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let mut table = self.table.write().await;
        match table.entry(key.into()) {
            Vacant(_) => return Err(anyhow!("Record '{}' is missing", key)),
            Occupied(mut entry) => {
                if entry.get() == value {
                    return Err(anyhow!("Record '{}' already exists and identical", key));
                }
                *entry.get_mut() = value.into();
            }
        };
        Ok(String::default())
    }
}
