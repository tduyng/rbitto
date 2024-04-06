use super::Torrent;
use crate::utils::urlencode;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, Serialize)]
pub struct TrackerRequest {
    pub peer_id: String,
    pub port: u16,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub compact: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrackerResponse {
    pub interval: i64,
    pub peers: ByteBuf,
}

pub struct Tracker {}

impl Tracker {
    pub async fn get_peers(path: &str) -> Result<Vec<String>> {
        let peers = Self::get_peers_bytes_buf(path).await?;
        let mut addresses = Vec::new();

        for chunk in peers.chunks_exact(6) {
            let address = format!(
                "{}.{}.{}.{}:{}",
                chunk[0],
                chunk[1],
                chunk[2],
                chunk[3],
                ((chunk[4] as u16) << 8 | chunk[5] as u16)
            );
            addresses.push(address);
        }

        Ok(addresses)
    }

    async fn get_peers_bytes_buf(path: &str) -> Result<ByteBuf> {
        let torrent = Torrent::from_file(path)?;
        let info_hash = torrent.info_hash()?;

        let request = TrackerRequest {
            peer_id: "00112233445566778899".to_string(),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: torrent.info.length,
            compact: 1,
        };
        let request_params = serde_urlencoded::to_string(&request)?;

        let request_url = format!(
            "{}?{}&info_hash={}",
            torrent.announce,
            request_params,
            &urlencode(&info_hash)
        );
        let res = reqwest::get(request_url).await?.bytes().await?;
        let tracker_res = serde_bencode::from_bytes::<TrackerResponse>(&res)?;
        Ok(tracker_res.peers)
    }
}
