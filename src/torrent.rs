use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};
use std::{fs::File, io::Read};

/* Export modules */
mod handshake;
mod stream;
mod tracker;

pub use handshake::*;
pub use stream::*;
pub use tracker::*;

/* Start of code */
#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub length: usize,
    #[serde(rename = "piece length")]
    pub piece_length: usize,
    pub pieces: ByteBuf,
}

impl Torrent {
    pub fn from_file(path: &str) -> Result<Torrent> {
        let mut buffet = Vec::new();
        File::open(path)?.read_to_end(&mut buffet)?;

        Ok(serde_bencode::from_bytes(&buffet)?)
    }

    pub fn info_hash(&self) -> Result<[u8; 20]> {
        let info_bytes = serde_bencode::to_bytes(&self.info)?;
        let mut hasher = Sha1::new();
        hasher.update(&info_bytes);

        Ok(hasher.finalize().into())
    }

    pub fn piece_hashes(&self) -> Result<Vec<String>> {
        let piece_hashes = self.info.pieces[..]
            .chunks(20)
            .map(hex::encode)
            .collect::<Vec<String>>();

        Ok(piece_hashes)
    }
}
