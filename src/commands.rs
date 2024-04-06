use crate::{
    torrent::{Torrent, Tracker, TrackerRequest},
    utils::{self, urlencode},
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

    pub fn peers(path: &str) -> Result<()> {
        let torrent = Torrent::from_file(path)?;
        let info_hash = torrent.info_hash()?;

        // Ensure that the length of the byte slice is 20
        if info_hash.len() != 20 {
            return Err(anyhow::anyhow!("info_hash length is not 20 bytes"));
        }

        let info_hash_encoded = urlencode(&info_hash);

        let request = TrackerRequest {
            tracker_url: torrent.announce.clone(),
            info_hash: info_hash_encoded,
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
