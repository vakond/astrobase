//! astrobase-server config module.

pub const FAILURE: i32 = 1;
pub const DEFAULT_CONFIG: &str = "astrobase.json";
pub const DEFAULT_STORAGE: &str = "/tmp/astrobase.db";

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
    pub fn load(filename: &Path) -> Self {
        let text = read(filename).expect("Cannot read the config file");
        let cfg: Astrobase =
            serde_json::from_str(&text).expect("Invalid format of the config file");
        cfg
    }
}

/// Reads the main config from a file.
fn read(filename: &Path) -> anyhow::Result<String> {
    use anyhow::Context as _;
    let text = format!("{:?}", filename);
    Ok(std::fs::read_to_string(filename).context(text)?)
}
