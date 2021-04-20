//! astrobase-server persistent key-value database.

use super::storage::Storage;
use crate::config;
use anyhow::anyhow;
use async_trait::async_trait;
use file_lock::FileLock;
use std::path::{Path, PathBuf};

/// Represents the database internals.
pub struct Persistent {
    filename: PathBuf,
}

#[async_trait]
impl super::Database for Persistent {
    /// Construct new instance of the database.
    fn new() -> Self {
        Persistent {
            filename: config::DEFAULT_STORAGE.into(),
        }
    }

    /// Deletes file with records.
    async fn clear(&self) -> anyhow::Result<()> {
        if self.filename.exists() {
            let file = lock_write(&self.filename)?;
            std::fs::remove_file(&self.filename)?;
            file.unlock()?;
        }
        Ok(())
    }

    /// Returns a value or error if not found.
    async fn get(&self, key: &str) -> anyhow::Result<String> {
        let file = lock_read(&self.filename)?;

        let mut value = String::default();
        if let Ok(storage) = Storage::open(&self.filename) {
            value = storage.find_last(key)?;
        }
        if value.is_empty() {
            return Err(anyhow!("Record '{}' is missing", key));
        }

        file.unlock()?;
        Ok(value)
    }

    /// Inserts new record if there was no such file or key.
    async fn insert(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let file = lock_write(&self.filename)?;

        // RAII block to close file
        {
            if let Ok(storage) = Storage::open(&self.filename) {
                if !storage.find_last(key)?.is_empty() {
                    return Err(anyhow!("Record '{}' already exists", key));
                }
            }
        }

        let mut storage = Storage::open_w(&self.filename)?;
        storage.push(key, value)?;

        file.unlock()?;
        Ok(String::default())
    }

    /// Deletes a record or returns error if was missing.
    async fn delete(&self, key: &str) -> anyhow::Result<String> {
        let file = lock_write(&self.filename)?;

        // RAII block to close file
        let value = {
            let missing = anyhow!("Record '{}' is missing already", key);
            match Storage::open(&self.filename) {
                Err(_) => return Err(missing),
                Ok(storage) => {
                    let value = storage.find_last(key)?;
                    if value.is_empty() {
                        return Err(missing);
                    }
                    value
                }
            }
        };

        let mut storage = Storage::open_w(&self.filename)?;
        storage.mark_deleted(key)?;

        file.unlock()?;
        Ok(value)
    }

    /// Updates record or returns error if the record was missing or identical.
    async fn update(&self, key: &str, value: &str) -> anyhow::Result<String> {
        let file = lock_write(&self.filename)?;

        // RAII block to close file
        let old_value = {
            let missing = anyhow!("Record '{}' is missing", key);
            match Storage::open(&self.filename) {
                Err(_) => return Err(missing),
                Ok(storage) => {
                    let old_value = storage.find_last(key)?;
                    if old_value.is_empty() {
                        return Err(missing);
                    }
                    old_value
                }
            }
        };

        if value == old_value {
            return Err(anyhow!("Record '{}' already exists and identical", key));
        }

        let mut storage = Storage::open_w(&self.filename)?;
        storage.push(key, value)?;

        file.unlock()?;
        Ok(String::default())
    }
}

/// Locks a file for writing.
fn lock_write(filename: &Path) -> anyhow::Result<FileLock> {
    lock_file(filename, true)
}

/// Locks a file for reading.
fn lock_read(filename: &Path) -> anyhow::Result<FileLock> {
    lock_file(filename, false)
}

/// Locks a file for reading or writing.
fn lock_file(filename: &Path, is_writable: bool) -> anyhow::Result<FileLock> {
    Ok(FileLock::lock(
        filename
            .to_str()
            .ok_or_else(|| anyhow!("Unsupported file name '{:?}'", filename))?,
        true,
        is_writable,
    )?)
}
