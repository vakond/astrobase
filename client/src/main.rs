//! astrobase-client main module.

#![forbid(unsafe_code)]
#![deny(warnings)]
#![allow(clippy::derive_partial_eq_without_eq)]

mod cli;
mod command;
mod config;

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
    let rt = tokio::runtime::Runtime::new()?;

    match app.cmd {
        cli::Command::Get { key } => {
            rt.block_on(command::get(app.endpoint, key))?;
        }
        cli::Command::Insert { key, value } => {
            rt.block_on(command::insert(app.endpoint, key, value))?;
        }
        cli::Command::Delete { key } => {
            rt.block_on(command::delete(app.endpoint, key))?;
        }
        cli::Command::Update { key, value } => {
            rt.block_on(command::update(app.endpoint, key, value))?;
        }
    }

    Ok(())
}
