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
    pub async fn get_peers(path: &str) -> Result<ByteBuf> {
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
