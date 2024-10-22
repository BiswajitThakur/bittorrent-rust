use std::path::PathBuf;

use bittorrent_starter_rust::{decode_bencoded_value, parse_torrent_file, Keys};
use clap::{Parser, Subcommand};
use sha1::{Digest, Sha1};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Decode bencoded
    Decode { value: String },
    /// Parse torrent file
    Info { torrent: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Decode { value } => {
            let decoded_value = decode_bencoded_value(&value);
            println!("{}", decoded_value.0);
        }
        Command::Info { torrent } => {
            let t = parse_torrent_file(torrent);
            println!("Tracker URL: {}", t.announce);
            match t.info.keys {
                Keys::SingleFile { length } => println!("Length: {}", length),
                _ => {}
            }
            let info = serde_bencode::to_bytes(&t.info).unwrap();
            let mut hasher = Sha1::new();
            hasher.update(&info);
            let info_hash = hasher.finalize();
            println!("Info Hash: {}", hex::encode(&info_hash));
        }
    }
}
