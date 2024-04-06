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
    Decode {
        data: String,
    },
    Info {
        path: String,
    },
    Peers {
        path: String,
    },
    #[clap(name = "handshake")]
    HandShake {
        path: String,
        peer_address: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Decode { data } => Commands::decode(&data),
        Command::Info { path } => Commands::info(&path),
        Command::Peers { path } => Commands::peers(&path).await,
        Command::HandShake { path, peer_address } => {
            Commands::handshake(&path, &peer_address).await
        }
    }
}
