//! astrobase-client commands implementation.

pub mod api {
    tonic::include_proto!("api");
}

use api::{astrobase_client, Key, Pair};
use tonic::Request;
use tracing::info;

/// Calls RPC-method `Get`.
pub async fn get(endpoint: String, key: String) -> anyhow::Result<()> {
    info!("get");

    let req = Request::new(Key { key });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.get(req).await?.into_inner();
    dbg!(&resp);

    Ok(())
}

/// Calls RPC-method `Insert`.
pub async fn insert(endpoint: String, key: String, value: String) -> anyhow::Result<()> {
    info!("insert");

    let req = Request::new(Pair { key, value });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.insert(req).await?.into_inner();
    dbg!(&resp);

    Ok(())
}

/// Calls RPC-method `Delete`.
pub async fn delete(endpoint: String, key: String) -> anyhow::Result<()> {
    info!("delete");

    let req = Request::new(Key { key });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.delete(req).await?.into_inner();
    dbg!(&resp);

    Ok(())
}

/// Calls RPC-method `Update`.
pub async fn update(endpoint: String, key: String, value: String) -> anyhow::Result<()> {
    info!("update");

    let req = Request::new(Pair { key, value });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.update(req).await?.into_inner();
    dbg!(&resp);

    Ok(())
}
