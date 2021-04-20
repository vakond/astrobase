//! astrobase-server in-memory key-value database.

use super::{Error, Result};

use async_trait::async_trait;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Represents the database internals.
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

    /// Deletes all records.
    async fn clear(&self) -> Result<()> {
        let mut table = self.table.write().await;
        table.clear();
        Ok(())
    }

    /// Returns a value or error.
    async fn get(&self, key: &str) -> Result<String> {
        let table = self.table.read().await;
        let value = table
            .get(key)
            .ok_or_else(|| Error::RecordMissing(key.into()))?;
        Ok(value.clone())
    }

    /// Inserts new record if there was no such key or returns error.
    async fn insert(&self, key: &str, value: &str) -> Result<String> {
        let mut table = self.table.write().await;
        match table.entry(key.into()) {
            Occupied(_) => return Err(Error::RecordAlreadyExists(key.into())),
            Vacant(entry) => entry.insert(value.into()),
        };
        Ok(String::default())
    }

    /// Deletes a record or returns error if was missing.
    async fn delete(&self, key: &str) -> Result<String> {
        let mut table = self.table.write().await;
        let value = table
            .remove(key)
            .ok_or_else(|| Error::RecordAlreadyMissing(key.into()))?;
        Ok(value.clone())
    }

    /// Updates record or returns error if the record was missing or identical.
    async fn update(&self, key: &str, value: &str) -> Result<String> {
        let mut table = self.table.write().await;
        match table.entry(key.into()) {
            Vacant(_) => return Err(Error::RecordMissing(key.into())),
            Occupied(mut entry) => {
                if entry.get() == value {
                    return Err(Error::RecordAlreadyExistsIdentical(key.into()));
                }
                *entry.get_mut() = value.into();
            }
        };
        Ok(String::default())
    }
}
