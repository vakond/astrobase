//! astrobase-server server implementation.

pub mod api {
    tonic::include_proto!("api");
}

use crate::config;
use api::{astrobase_server, Key, Output, Pair};
use std::sync::Arc;
use std::time::Duration;
use tonic::{transport, Request, Response, Status};
use tracing::info;

/// Starts the server in listening mode.
pub async fn run(cfg: config::Astrobase) -> anyhow::Result<()> {
    let service = Service::new();

    let stats = service.stats.clone();
    let interval = Duration::from_secs(cfg.ticker.interval);
    tokio::spawn(async move {
        let interrupted = false;
        while !interrupted {
            tokio::time::sleep(interval).await;
            stats.dump();
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

/// Represents the gRPC service.
#[derive(Debug, Default)]
struct Statistics {
    number_of_records: u64,
}

impl Statistics {
    fn dump(&self) {
        info!("{:?}", self);
    }
}

/// Represents the gRPC service.
struct Service {
    stats: Arc<Statistics>,
}

impl Service {
    fn new() -> Self {
        Service {
            stats: Arc::new(Statistics::default()),
        }
    }
}

type CallResult = Result<Response<Output>, Status>;

#[tonic::async_trait]
impl astrobase_server::Astrobase for Service {
    /// Handles command "Get".
    async fn get(&self, _req: Request<Key>) -> CallResult {
        Ok(Response::new(Output {
            ok: true,
            info: "XXXXXXXXX".into(),
        }))
    }

    /// Handles command "Insert".
    async fn insert(&self, _req: Request<Pair>) -> CallResult {
        Ok(Response::new(Output {
            ok: true,
            info: "YYYYYYYYYYYY".into(),
        }))
    }

    /// Handles command "Delete".
    async fn delete(&self, _req: Request<Key>) -> CallResult {
        Ok(Response::new(Output {
            ok: true,
            info: "ZZZZZZZZZZZ".into(),
        }))
    }

    /// Handles command "Update".
    async fn update(&self, _req: Request<Pair>) -> CallResult {
        Ok(Response::new(Output {
            ok: true,
            info: "TTTTTTTTTTTT".into(),
        }))
    }
}
