//! astrobase-server key-value database.

mod inmemory;
mod persistent;
mod storage;

#[cfg(test)]
mod tests;

pub use inmemory::InMemory;
pub use persistent::Persistent;

use async_trait::async_trait;

/// Represents interface of the database.
#[async_trait]
pub trait Database: Send + Sync + 'static {
    fn new() -> Self;
    async fn clear(&self) -> Result<()>;
    async fn get(&self, key: &str) -> Result<String>;
    async fn insert(&self, key: &str, value: &str) -> Result<String>;
    async fn delete(&self, key: &str) -> Result<String>;
    async fn update(&self, key: &str, value: &str) -> Result<String>;
}

use std::path::PathBuf;

/// Represents errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Record '{0}' is missing")]
    RecordMissing(String),
    #[error("Record '{0}' is already missing")]
    RecordAlreadyMissing(String),
    #[error("Record '{0}' already exists")]
    RecordAlreadyExists(String),
    #[error("Record '{0}' already exists and identical")]
    RecordAlreadyExistsIdentical(String),
    #[error("Invalid record '{0}'")]
    RecordInvalid(String),

    #[error("Unsupported file name '{0}'")]
    Filename(PathBuf),

    #[error("Cannot open file '{1}': {0}")]
    OpenFile(#[source] std::io::Error, PathBuf),
    #[error("Cannot delete file '{1}': {0}")]
    DeleteFile(#[source] std::io::Error, PathBuf),
    #[error("Cannot lock file '{1}': {0}")]
    LockFile(#[source] std::io::Error, PathBuf),
    #[error("Cannot unlock file '{1}': {0}")]
    UnlockFile(#[source] std::io::Error, PathBuf),

    #[error("Input/output error")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
