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
    let rt = tokio::runtime::Runtime::new()?;

    match app.cmd {
        cli::Command::Get => {
            rt.block_on(command::get())?;
        }
        cli::Command::Insert => {
            rt.block_on(command::insert())?;
        }
        cli::Command::Delete => {
            rt.block_on(command::delete())?;
        }
        cli::Command::Update => {
            rt.block_on(command::update())?;
        }
    }

    Ok(())
}
