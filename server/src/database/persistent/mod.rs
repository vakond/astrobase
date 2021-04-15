//! astrobase-server persistent key-value database.

use crate::config;

use async_trait::async_trait;
use std::path::PathBuf;
use tokio::sync::Mutex;

/// Represents the database internal structure.
#[allow(unused)]
pub struct Persistent {
    storage: Mutex<PathBuf>,
}

#[async_trait]
impl super::Database for Persistent {
    /// Construct new instance of the database.
    fn new() -> Self {
        Persistent {
            storage: Mutex::new(config::DEFAULT_STORAGE.into()),
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
