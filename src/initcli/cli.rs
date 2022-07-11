use clap::{IntoApp, Parser, Subcommand};

use crate::log;

use super::{disable, enable, manpage};

// get version from file Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(
    name = "initrs-cli",
    about = "Command Line Interface for Initrs",
    version = VERSION,
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "Add the service at startup", display_order = 1)]
    Enable { service: String },
    #[clap(about = "Remove the service from startup", display_order = 2)]
    Disable { service: String },
    #[clap(about = "Generate man page", display_order = 3)]
    Manpage,
}

pub fn run() {
    let args = Cli::parse();

    let result = match args.command {
        Command::Enable { service } => enable(&service),
        Command::Disable { service } => disable(&service),
        Command::Manpage => manpage(Cli::command()),
    };

    if let Err(err) = result {
        log::error!("Error: {err}")
    }
}
