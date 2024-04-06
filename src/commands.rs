use crate::{
    torrent::{Torrent, Tracker}, utils,
};
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

        // Convert the info hash byte array to a hexadecimal string
        let info_hash_hex = hex::encode(torrent.info_hash()?);
        println!("Info Hash: {}", info_hash_hex);
        println!("Piece Length: {}", torrent.info.piece_length);
        println!("Piece Hashes:");
        for hash in torrent.piece_hashes()? {
            println!("{}", hash);
        }
        Ok(())
    }

    pub async fn peers(path: &str) -> Result<()> {
        let peers = Tracker::get_peers(path).await?;

        println!("Peers:");
        for peer in peers {
            println!("{}", peer);
        }
        Ok(())
    }
}
