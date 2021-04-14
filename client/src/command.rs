//! astrobase-client commands implementation.

use tracing::info;

///
pub async fn get() -> anyhow::Result<()> {
    info!("get");
    Ok(())
}

///
pub async fn insert() -> anyhow::Result<()> {
    info!("insert");
    Ok(())
}

///
pub async fn delete() -> anyhow::Result<()> {
    info!("delete");
    Ok(())
}

///
pub async fn update() -> anyhow::Result<()> {
    info!("update");
    Ok(())
}
