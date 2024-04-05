use bittorrent_starter_rust::cli::decode::decode_bencoded_value;
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
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Decode { data } => {
            let decoded_value = decode_bencoded_value(&data);
            println!("{}", decoded_value);
        }
    }
}
