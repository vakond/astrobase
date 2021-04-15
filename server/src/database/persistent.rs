//! astrobase-server persistent key-value database.

use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Represents the database internal structure.
#[allow(unused)]
pub struct Persistent {
    table: RwLock<HashMap<String, String>>,
}

#[async_trait]
impl super::Database for Persistent {
    /// Construct new instance of the database.
    fn new() -> Self {
        Persistent {
            table: RwLock::new(HashMap::new()),
        }
    }

    /// Returns a value or error.
    async fn get(&self, _key: &str) -> anyhow::Result<String> {
        unimplemented!();
    }

    /// Inserts new record if there was no such key or returns error.
    async fn insert(&self, _key: &str, _value: &str) -> anyhow::Result<String> {
        unimplemented!();
    }

    /// Deletes a record or returns error if was missing.
    async fn delete(&self, _key: &str) -> anyhow::Result<String> {
        unimplemented!();
    }

    /// Updates record or returns error if the record was missing or identical.
    async fn update(&self, _key: &str, _value: &str) -> anyhow::Result<String> {
        unimplemented!();
    }
}
