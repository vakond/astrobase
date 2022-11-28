//! astrobase-server implementation.

mod api {
    tonic::include_proto!("api");
}

use crate::stats::Stats;
use crate::{config, database, database::Database};

use api::{astrobase_server, Key, Output, Pair};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tonic::{transport, Request, Response, Status};
use tracing::info;

/// Starts the server in listening mode plus task for monitoring.
pub async fn run(cfg: config::Astrobase) -> anyhow::Result<()> {
    use anyhow::Context as _;

    #[cfg(feature = "inmemory")]
    let service = Service::<database::InMemory>::new();
    #[cfg(feature = "persistent")]
    let service = Service::<database::Persistent>::new();

    start_monitoring(
        service.stats.clone(),
        Duration::from_secs(cfg.monitoring.interval),
    );

    info!("Ready");
    let endpoint = cfg.server.endpoint.clone();
    transport::Server::builder()
        .add_service(astrobase_server::AstrobaseServer::new(service))
        .serve(endpoint.parse().context(endpoint)?)
        .await?;

    Ok(())
}

/// Launches additional task which dumps the statistics regularly.
fn start_monitoring(stats: Arc<RwLock<Stats>>, interval: Duration) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            stats.read().await.dump();
        }
    });
}

/// Represents the `gRPC` service.
struct Service<Db: Database> {
    db: Db,
    stats: Arc<RwLock<Stats>>,
}

impl<Db: Database> Service<Db> {
    fn new() -> Self {
        Service {
            db: Db::new(),
            stats: Arc::new(RwLock::new(Stats::default())),
        }
    }
}

type CallResult = Result<Response<Output>, Status>;

#[tonic::async_trait]
impl<Db: Database> astrobase_server::Astrobase for Service<Db> {
    /// Handles command "Get".
    async fn get(&self, req: Request<Key>) -> CallResult {
        let key = &req.get_ref().key;
        let r = self.db.get(key).await;
        let ok = r.is_ok();
        self.stats.write().await.get(ok);
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
        self.stats.write().await.insert(ok);
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
        self.stats.write().await.delete(ok);
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
        self.stats.write().await.update(ok);
        let info = if ok {
            r.unwrap()
        } else {
            r.unwrap_err().to_string()
        };
        Ok(Response::new(Output { ok, info }))
    }
}
