//! astrobase-server persistent key-value database.

mod storage;
#[cfg(test)]
mod test;

use crate::config;
use anyhow::anyhow;
use async_trait::async_trait;
use std::path::PathBuf;
use storage::Storage;
use tokio::sync::Mutex;

/// Represents the database internals.
pub struct Persistent {
    filename: Mutex<PathBuf>,
}

#[async_trait]
impl super::Database for Persistent {
    /// Construct new instance of the database.
    fn new() -> Self {
        Persistent {
            filename: Mutex::new(config::DEFAULT_STORAGE.into()),
        }
    }

    /// Returns a value or error.
    async fn get(&self, key: &str) -> anyhow::Result<String> {
        let filename = self.filename.lock().await;
        let mut value = String::default();
        if let Ok(storage) = Storage::open(&filename) {
            value = storage.find_last(key)?;
        }
        if value.is_empty() {
            return Err(anyhow!("Record '{}' is missing", key));
        }
        Ok(value)
    }

    /// Inserts new record if there was no such file or key.
    async fn insert(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let filename = self.filename.lock().await;
        // RAII block
        {
            if let Ok(storage) = Storage::open(&filename) {
                if storage.contains(key)? {
                    return Err(anyhow!("Record '{}' already exists", key));
                }
            }
        }
        let mut storage = Storage::open_w(&filename)?;
        storage.append(key, value)?;
        Ok(String::default())
    }

    /// Deletes a record or returns error if was missing.
    async fn delete(&self, key: &str) -> anyhow::Result<String> {
        let filename = self.filename.lock().await;
        // RAII block
        let value = {
            let missing = anyhow!("Record '{}' is missing already", key);
            match Storage::open(&filename) {
                Err(_) => return Err(missing),
                Ok(storage) => {
                    let value = storage.find_last(key)?;
                    if value.is_empty() || value == storage::DELETED {
                        return Err(missing);
                    }
                    value
                }
            }
        };
        let mut storage = Storage::open_w(&filename)?;
        storage.mark_deleted(key)?;
        Ok(value)
    }

    /// Updates record or returns error if the record was missing or identical.
    async fn update(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let filename = self.filename.lock().await;

        // RAII block
        let old_value = {
            let missing = anyhow!("Record '{}' is missing", key);
            match Storage::open(&filename) {
                Err(_) => return Err(missing),
                Ok(storage) => {
                    let old_value = storage.find_last(key)?;
                    if old_value.is_empty() || value == storage::DELETED {
                        return Err(missing);
                    }
                    old_value
                }
            }
        };

        if value == old_value {
            return Err(anyhow!("Record '{}' already exists and identical", key));
        }

        let mut storage = Storage::open_w(&filename)?;
        storage.append(key, value)?;
        Ok(String::default())
    }
}
