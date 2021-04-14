//! astrobase-client main module.

#![forbid(unsafe_code)]
#![deny(warnings)]

mod cli;
mod command;

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
    let endpoint = app.endpoint.unwrap_or("http://[::1]:50051".into());
    let rt = tokio::runtime::Runtime::new()?;

    match app.cmd {
        cli::Command::Get { key } => {
            rt.block_on(command::get(endpoint, key))?;
        }
        cli::Command::Insert { key, value } => {
            rt.block_on(command::insert(endpoint, key, value))?;
        }
        cli::Command::Delete { key } => {
            rt.block_on(command::delete(endpoint, key))?;
        }
        cli::Command::Update { key, value } => {
            rt.block_on(command::update(endpoint, key, value))?;
        }
    }

    Ok(())
}
