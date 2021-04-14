//! astrobase-server implementation.

pub mod api {
    tonic::include_proto!("api");
}

use crate::config;
use api::{astrobase_server, Key, Output, Pair};
use tonic::{transport, Request, Response, Status};
use tracing::info;

/// Starts the server in listening mode.
pub async fn run(cfg: config::Astrobase) -> anyhow::Result<()> {
    let service = Service::new();

    let stats = service.stats.clone();
    let interval = std::time::Duration::from_secs(cfg.ticker.interval);
    tokio::spawn(async move {
        let interrupted = false;
        while !interrupted {
            tokio::time::sleep(interval).await;
            stats.dump().await;
        }
    });

    info!("Ready");
    let endpoint = cfg.server.endpoint.clone();
    transport::Server::builder()
        .add_service(astrobase_server::AstrobaseServer::new(service))
        .serve(endpoint.parse()?)
        .await?;

    Ok(())
}

use crate::database::Database;
use crate::stats::Stats;
use std::sync::Arc;

/// Represents the gRPC service.
struct Service {
    db: Database,
    stats: Arc<Stats>,
}

impl Service {
    fn new() -> Self {
        Service {
            db: Database::new(),
            stats: Arc::new(Stats::new()),
        }
    }
}

type CallResult = Result<Response<Output>, Status>;

#[tonic::async_trait]
impl astrobase_server::Astrobase for Service {
    /// Handles command "Get".
    async fn get(&self, req: Request<Key>) -> CallResult {
        let key = &req.get_ref().key;
        let r = self.db.get(key).await;
        let ok = r.is_ok();
        self.stats.get_ok(ok).await;
        let info = if ok {
            r.unwrap()
        } else {
            r.unwrap_err().to_string()
        };
        Ok(Response::new(Output { ok, info }))
    }

    /// Handles command "Insert".
    async fn insert(&self, req: Request<Pair>) -> CallResult {
        let key = &req.get_ref().key;
        let value = &req.get_ref().value;
        let r = self.db.insert(key, value).await;
        let ok = r.is_ok();
        self.stats.insert_ok(ok).await;
        let info = if ok {
            r.unwrap()
        } else {
            r.unwrap_err().to_string()
        };
        Ok(Response::new(Output { ok, info }))
    }

    /// Handles command "Delete".
    async fn delete(&self, req: Request<Key>) -> CallResult {
        let key = &req.get_ref().key;
        let r = self.db.delete(key).await;
        let ok = r.is_ok();
        self.stats.delete_ok(ok).await;
        let info = if ok {
            r.unwrap()
        } else {
            r.unwrap_err().to_string()
        };
        Ok(Response::new(Output { ok, info }))
    }

    /// Handles command "Update".
    async fn update(&self, req: Request<Pair>) -> CallResult {
        let key = &req.get_ref().key;
        let value = &req.get_ref().value;
        let r = self.db.update(key, value).await;
        let ok = r.is_ok();
        self.stats.update_ok(ok).await;
        let info = if ok {
            r.unwrap()
        } else {
            r.unwrap_err().to_string()
        };
        Ok(Response::new(Output { ok, info }))
    }
}
