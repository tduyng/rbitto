use std::path::PathBuf;

use anyhow::Result;
use bittorrent_starter_rust::commands::Commands;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Decode { data: String },
    Info {file: PathBuf}
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Decode { data } => Commands::decode(&data),
        Command::Info { file } => Commands::info(file),
    }
}
