//! astrobase-server main module.

#![forbid(unsafe_code)]
#![deny(warnings)]
#![allow(clippy::derive_partial_eq_without_eq)]

mod cli;
mod config;
mod database;
mod server;
mod stats;

fn main() {
    init_logger();
    if let Err(err) = execute(cli::application()) {
        eprintln!("Error: {:#}", err);
        std::process::exit(config::FAILURE);
    }
}

/// Initializes the logger.
fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();
}

/// Dispatches CLI commands.
fn execute(app: cli::Application) -> anyhow::Result<()> {
    match app.cmd {
        cli::Command::Run => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(run(&app.config))?;
        }
    }

    tracing::info!("Done.");
    Ok(())
}

/// Runs the server.
async fn run(config_file: &std::path::Path) -> anyhow::Result<()> {
    let cfg = config::Astrobase::load(config_file)?;
    server::run(cfg).await
}
