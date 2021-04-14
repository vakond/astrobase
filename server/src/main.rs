//! astrobase-server main module.

#![forbid(unsafe_code)]
#![deny(warnings)]

mod cli;
mod config;
mod server;

fn main() {
    init_logger();
    if let Err(err) = execute(cli::application()) {
        eprintln!("Error: {:#}", err);
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
            let config_file = app.config.unwrap_or("astrobase.json".into());
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(run(&config_file))?;
        }
    }

    tracing::info!("Done.");
    Ok(())
}

/// Runs the server.
async fn run(config_file: &std::path::Path) -> anyhow::Result<()> {
    let cfg = config::Astrobase::load(config_file);
    server::run(cfg).await?;
    Ok(())
}
