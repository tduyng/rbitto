use std::{fs::File, io::Read};
use anyhow::Result;
use serde::Deserialize;
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
    pub length: u64,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    pub pieces: ByteBuf,
}

impl Torrent {
    pub fn from_file(path: &str) -> Result<Torrent> {
        let mut buffet = Vec::new();
        File::open(path)?.read_to_end(&mut buffet)?;
        
        Ok(serde_bencode::from_bytes(&buffet)?)
    }
}
