//! astrobase-server server implementation.

use crate::config;
use tracing::info;

/// Starts the server in listening mode.
pub async fn run(_cfg: &config::Server) -> anyhow::Result<()> {
    info!("Server");
    Ok(())
}
