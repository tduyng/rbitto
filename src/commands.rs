use crate::{
    torrent::{Torrent, Tracker, TrackerRequest},
    utils,
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
        println!("Info Hash: {}", torrent.info_hash()?);
        println!("Piece Length: {}", torrent.info.piece_length);
        println!("Piece Hashes:");
        for hash in torrent.piece_hashes()? {
            println!("{}", hash);
        }
        Ok(())
    }

    pub fn peers(path: &str) -> Result<()> {
        let torrent = Torrent::from_file(path)?;
        
        let request = TrackerRequest {
            tracker_url: torrent.announce.clone(),
            info_hash: torrent.info_hash()?,
            peer_id: "00112233445566778899".to_string(),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: torrent.info.length,
            compact: 1,
        };
        let peers = Tracker::get_peers(request)?;

        println!("Peers:");
        for peer in peers {
            println!("{}", peer);
        }
        Ok(())
    }
}
