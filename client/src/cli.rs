//! astrobase-client options parser.

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Get value by key")]
    Get,

    #[structopt(about = "Insert new record")]
    Insert,

    #[structopt(about = "Delete record by key")]
    Delete,

    #[structopt(about = "Update value by key")]
    Update,
}

#[derive(StructOpt)]
#[structopt(about = "Key-value database")]
pub struct Application {
    #[structopt(
        parse(from_os_str),
        short,
        long,
        help = "Endpoint of the server (default: http://[::1]:50051)"
    )]
    pub endpoint: Option<PathBuf>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

/// Constructs instance of Application.
pub fn application() -> Application {
    Application::from_args()
}
