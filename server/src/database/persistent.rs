//! astrobase-server persistent key-value database.

use super::storage::Storage;
use super::{Error, Result};
use crate::config;

use async_trait::async_trait;
use file_lock::FileLock;
use std::path::{Path, PathBuf};

/// Represents the database internals.
pub struct Persistent {
    filename: PathBuf,
    //index: HashMap<String, u64>,
}

#[async_trait]
impl super::Database for Persistent {
    /// Construct new instance of the database.
    fn new() -> Self {
        Persistent {
            filename: config::DEFAULT_DB.into(),
            //index: HashMap::new(),
        }
    }

    /// Deletes file with records.
    async fn clear(&self) -> Result<()> {
        if !self.filename.exists() {
            return Ok(());
        }

        let file = lock_write(&self.filename)?;
        std::fs::remove_file(&self.filename)
            .map_err(|e| Error::DeleteFile(e, self.filename.clone()))?;

        file.unlock()?;
        Ok(())
    }

    /// Returns a value or error if not found.
    async fn get(&self, key: &str) -> Result<String> {
        if !self.filename.exists() {
            return Err(Error::FileMissing(self.filename.clone()));
        }

        let mut value = String::default();
        let file = lock_read(&self.filename)?;

        // RAII block to close file
        {
            if let Ok(storage) = Storage::open(&self.filename) {
                value = storage.find_last(key)?;
            }
        }

        if value.is_empty() {
            return Err(Error::RecordMissing(key.into()));
        }

        file.unlock()?;
        Ok(value)
    }

    /// Inserts new record if there was no such file or key.
    async fn insert(&self, key: &str, value: &str) -> Result<String> {
        let file = lock_write(&self.filename)?;

        // RAII block to close file
        {
            if let Ok(storage) = Storage::open(&self.filename) {
                if !storage.find_last(key)?.is_empty() {
                    return Err(Error::RecordAlreadyExists(key.into()));
                }
            }
        }

        // RAII block to close file
        {
            let mut storage = Storage::open_w(&self.filename)?;
            storage.push(key, value)?;
        }

        file.unlock()?;
        Ok(String::default())
    }

    /// Deletes a record or returns error if was missing.
    async fn delete(&self, key: &str) -> Result<String> {
        let file = lock_write(&self.filename)?;

        // RAII block to close file
        let value = {
            match Storage::open(&self.filename) {
                Err(_) => return Err(Error::RecordAlreadyMissing(key.into())),
                Ok(storage) => {
                    let value = storage.find_last(key)?;
                    if value.is_empty() {
                        return Err(Error::RecordAlreadyMissing(key.into()));
                    }
                    value
                }
            }
        };

        // RAII block to close file
        {
            let mut storage = Storage::open_w(&self.filename)?;
            storage.mark_deleted(key)?;
        }

        file.unlock()?;
        Ok(value)
    }

    /// Updates record or returns error if the record was missing or identical.
    async fn update(&self, key: &str, value: &str) -> Result<String> {
        let file = lock_write(&self.filename)?;

        // RAII block to close file
        let old_value = {
            match Storage::open(&self.filename) {
                Err(_) => return Err(Error::RecordMissing(key.into())),
                Ok(storage) => {
                    let old_value = storage.find_last(key)?;
                    if old_value.is_empty() {
                        return Err(Error::RecordMissing(key.into()));
                    }
                    old_value
                }
            }
        };

        if value == old_value {
            return Err(Error::RecordAlreadyExistsIdentical(key.into()));
        }

        // RAII block to close file
        {
            let mut storage = Storage::open_w(&self.filename)?;
            storage.push(key, value)?;
        }

        file.unlock()?;
        Ok(String::default())
    }
}

/// Locks a file for writing.
fn lock_write(filename: &Path) -> Result<FileLock> {
    lock_file(filename, true)
}

/// Locks a file for reading.
fn lock_read(filename: &Path) -> Result<FileLock> {
    lock_file(filename, false)
}

/// Locks a file for reading or writing.
fn lock_file(filename: &Path, is_writable: bool) -> Result<FileLock> {
    FileLock::lock(
        filename
            .to_str()
            .ok_or_else(|| Error::Filename(filename.into()))?,
        true,
        is_writable,
    )
    .map_err(|e| Error::LockFile(e, filename.into()))
}
