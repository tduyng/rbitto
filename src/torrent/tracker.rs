use anyhow::Result;
use serde::Deserialize;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TrackerRequest {
    pub tracker_url: String,
    pub info_hash: String,
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u16,
    pub downloaded: u64,
    pub left: u64,
    pub compact: u64,
}

#[derive(Debug, Deserialize)]
pub struct TrackerResponse {
    pub interval: i64,
    pub peers: Vec<String>,
}

impl Display for TrackerResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Peers:\n{}", self.peers.join("\n"))
    }
}

pub struct Tracker {}

impl Tracker {
    pub async fn get_peers(request: TrackerRequest) -> Result<Vec<String>> {
        let request_params = serde_urlencoded::to_string([
            ("info_hash", &request.info_hash),
            ("peer_id", &request.peer_id),
            ("port", &request.port.to_string()),
            ("downloaded", &request.downloaded.to_string()),
            ("left", &request.left.to_string()),
            ("uploaded", &request.uploaded.to_string()),
            ("compact", &request.compact.to_string()),
        ])?;

        let request_url = format!("{}?{}", request.tracker_url, request_params);
        let res = reqwest::get(&request_url).await?.bytes().await?;
        let tracker_res = serde_bencode::from_bytes::<TrackerResponse>(&res)?;
        Ok(tracker_res.peers)
    }
}
