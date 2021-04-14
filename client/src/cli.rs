//! astrobase-client options parser.

use crate::config;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Key-value database")]
pub struct Application {
    #[structopt(
        parse(from_str),
        short,
        long,
        default_value = &config::DEFAULT_ENDPOINT,
        help = "The service endpoint"
    )]
    pub endpoint: String,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Get value by key")]
    Get { key: String },

    #[structopt(about = "Insert new record")]
    Insert { key: String, value: String },

    #[structopt(about = "Delete record by key")]
    Delete { key: String },

    #[structopt(about = "Update value by key")]
    Update { key: String, value: String },
}

/// Constructs an instance of the Application.
pub fn application() -> Application {
    Application::from_args()
}
