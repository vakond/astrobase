//! astrobase-server monitoring implementation.

use crate::config;
use tracing::info;

/// Starts producing output of the statistics.
pub async fn run(_cfg: &config::Monitoring) -> anyhow::Result<()> {
    info!("Monitoring");
    Ok(())
}
