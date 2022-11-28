//! astrobase-server config module.

pub const FAILURE: i32 = 1;
pub const DEFAULT_CONFIG: &str = "astrobase.json";
pub const DEFAULT_DB: &str = "/tmp/astrobase.db";
//pub const DEFAULT_INDEX: &str = "/tmp/astrobase.idx";

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents the server config.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Server {
    pub endpoint: String,
}

/// Represents the monitoring config.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Monitoring {
    pub interval: u64, // seconds
}

/// Represents the main config.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Astrobase {
    pub environment: String,
    pub server: Server,
    pub monitoring: Monitoring,
}

/// Implements construction of the config.
impl Astrobase {
    pub fn load(filename: &Path) -> Result<Self> {
        let text = read(filename)?;
        let cfg: Astrobase =
            serde_json::from_str(&text).map_err(|e| Error::Parse(e, filename.to_owned()))?;
        Ok(cfg)
    }
}

/// Reads the main config from a file.
fn read(filename: &Path) -> Result<String> {
    std::fs::read_to_string(filename).map_err(|e| Error::Read(e, filename.to_owned()))
}

/// Represents config errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read config '{1}': {0}")]
    Read(#[source] std::io::Error, std::path::PathBuf),
    #[error("Failed to parse config '{1}': {0}")]
    Parse(#[source] serde_json::Error, std::path::PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;
