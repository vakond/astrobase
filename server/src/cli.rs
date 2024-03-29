//! astrobase-server options parser.

use crate::config;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Astrobase key-value database server")]
pub struct Application {
    #[structopt(
        parse(from_os_str),
        short,
        long,
        default_value = &config::DEFAULT_CONFIG,
        help = "Path to the config file"
    )]
    pub config: PathBuf,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Starts listening")]
    Run,
}

/// Constructs instance of Application.
pub fn application() -> Application {
    Application::from_args()
}
