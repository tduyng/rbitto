use crate::{torrent::Torrent, utils};
use anyhow::Result;

pub struct Commands {}

impl Commands {
    pub fn decode(data: &str) -> Result<()> {
        let decoded_value = utils::decode(data)?;

        println!("{}", decoded_value);
        Ok(())
    }

    pub fn info(path: &str) -> Result<()> {
        let torrent = Torrent::from_file(path)?;
        println!("Tracker URL: {}", torrent.announce);
        println!("Length: {}", torrent.info.length);
        println!("Info Hash: {}", torrent.info_hash()?);
        println!("Piece Length: {}", torrent.info.piece_length);
        println!("Piece Hashes:");
        for hash in torrent.piece_hashes()? {
            println!("{}", hash);
        }
        Ok(())
    }
}
