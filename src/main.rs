use anyhow::Result;
use rbitto::commands::Commands;
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
    #[clap(name = "download_piece")]
    DownloadPiece {
        #[arg(short, long)]
        output: String,
        path: String,
        piece_index: u32,
    },
    Download {
        #[arg(short, long)]
        output: String,
        path: String,
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
        Command::DownloadPiece {
            output,
            path,
            piece_index,
        } => Commands::download_piece(&output, &path, piece_index).await,
        Command::Download { output, path } => Commands::download(&output, &path).await,
    }
}
