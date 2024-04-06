use crate::{
    torrent::{HandShake, Stream, Torrent, Tracker, HANDSHAKE_BUF_INDEX_START},
    utils,
};
use anyhow::{Ok, Result};
use tokio::fs;

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
        for address in peers {
            println!("{}", address);
        }

        Ok(())
    }

    pub async fn handshake(path: &str, peer_address: &str) -> Result<()> {
        let torrent = Torrent::from_file(path)?;
        let info_hash = torrent.info_hash()?;
        let handshake = HandShake::new(info_hash);
        let mut stream = Stream::connect(peer_address).await?;
        let response = stream.handshake(handshake).await?;
        let peer_id = &response[HANDSHAKE_BUF_INDEX_START..];
        let peer_id_hex = hex::encode(peer_id);

        println!("Peer ID: {}", peer_id_hex);
        Ok(())
    }

    pub async fn download_piece(output: &str, path: &str, piece_index: u32) -> Result<()> {
        let torrent = Torrent::from_file(path)?;
        let info_hash = torrent.info_hash()?;
        let peers = Tracker::get_peers(path).await?;
        let mut stream = Stream::connect(&peers[0]).await?;
        let handshake = HandShake::new(info_hash);
        stream.handshake(handshake).await?;
        stream.bitfield().await?;
        stream.interested().await?;
        stream.wait_unchoke().await?;

        let piece_data: Vec<u8> = stream.get_piece_data(piece_index, &torrent).await?;
        // let mut hasher = <Sha1 as Digest>::new();
        // hasher.update(&piece_data);
        // let piece_hash: [u8; 20] = hasher.finalize().into();

        // let torrent_hash = &torrent.info.pieces.0[piece_index as usize];
        // if &piece_hash != torrent_hash {
        //     panic!("Hashes do NOT match!");
        // }

        fs::write(output, &piece_data).await?;
        Ok(())
    }
}
