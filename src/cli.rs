use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Enigma Keyring")]
pub(crate) struct Args {
    #[arg(short, long, default_value = "config.toml")]
    pub(crate) config: PathBuf,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Create { label: String },
    List,
    Get { label: String },
    Delete { label: String },
}
