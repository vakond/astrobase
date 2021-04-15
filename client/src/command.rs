//! astrobase-client gRPC API calls.

mod api {
    tonic::include_proto!("api");
}

use api::{astrobase_client, Key, Pair};
use tonic::Request;
use tracing::{info, warn};

/// Calls RPC-method `Get`.
pub async fn get(endpoint: String, key: String) -> anyhow::Result<()> {
    ensure_key_valid(&key)?;

    let req = Request::new(Key { key: key.clone() });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.get(req).await?.into_inner();
    if resp.ok {
        info!("key: {}, value: {}", key, resp.info);
    } else {
        warn!("{}", resp.info);
    }

    Ok(())
}

/// Calls RPC-method `Insert`.
pub async fn insert(endpoint: String, key: String, value: String) -> anyhow::Result<()> {
    ensure_key_valid(&key)?;
    ensure_value_valid(&value)?;

    let req = Request::new(Pair {
        key: key.clone(),
        value: value.clone(),
    });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.insert(req).await?.into_inner();
    if resp.ok {
        info!("key: {}, value: {}", key, value);
    } else {
        warn!("{}", resp.info);
    }

    Ok(())
}

/// Calls RPC-method `Delete`.
pub async fn delete(endpoint: String, key: String) -> anyhow::Result<()> {
    ensure_key_valid(&key)?;

    let req = Request::new(Key { key: key.clone() });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.delete(req).await?.into_inner();
    if resp.ok {
        info!("key: {}, value: {}", key, resp.info);
    } else {
        warn!("{}", resp.info);
    }

    Ok(())
}

/// Calls RPC-method `Update`.
pub async fn update(endpoint: String, key: String, value: String) -> anyhow::Result<()> {
    ensure_key_valid(&key)?;
    ensure_value_valid(&value)?;

    let req = Request::new(Pair {
        key: key.clone(),
        value: value.clone(),
    });
    let mut caller = astrobase_client::AstrobaseClient::connect(endpoint).await?;
    let resp = caller.update(req).await?.into_inner();
    if resp.ok {
        info!("key: {}, value: {}", key, value);
    } else {
        warn!("{}", resp.info);
    }

    Ok(())
}

use anyhow::anyhow;

/// Checks the length of a key is below the limit.
fn ensure_key_valid(key: &str) -> anyhow::Result<()> {
    if key.len() > crate::config::MAX_KEY_LEN {
        return Err(anyhow!("key is too long: {}", key.len()));
    }
    Ok(())
}

/// Checks the length of a value is below the limit.
fn ensure_value_valid(value: &str) -> anyhow::Result<()> {
    if value.len() > crate::config::MAX_VALUE_LEN {
        return Err(anyhow!("value is too long: {}", value.len()));
    }
    Ok(())
}
