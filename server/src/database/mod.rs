//! astrobase-server in-memory key-value database.

mod inmemory;

#[cfg(test)]
mod test;

pub use inmemory::InMemory;

use async_trait::async_trait;

/// Represents interface of the database.
#[async_trait]
pub trait Database: Send + Sync + 'static {
    fn new() -> Self;
    async fn get(&self, key: &str) -> anyhow::Result<String>;
    async fn insert(&self, key: &str, value: &str) -> anyhow::Result<String>;
    async fn delete(&self, key: &str) -> anyhow::Result<String>;
    async fn update(&self, key: &str, value: &str) -> anyhow::Result<String>;
}
