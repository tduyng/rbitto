use crate::{
    torrent::{Torrent, Tracker},
    utils,
};
use anyhow::{Ok, Result};
use serde_bytes::ByteBuf;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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
        let peers: ByteBuf = Tracker::get_peers(path).await?;

        println!("Peers:");
        for chunk in peers.chunks_exact(6) {
            println!(
                "{}.{}.{}.{}:{}",
                chunk[0],
                chunk[1],
                chunk[2],
                chunk[3],
                ((chunk[4] as u16) << 8 | chunk[5] as u16)
            );
        }

        Ok(())
    }

    pub async fn handshake(path: &str, peer_address: &str) -> Result<()> {
        let parts: Vec<&str> = peer_address.split(':').collect();
        let peer_ip = parts[0];
        let peer_port: u16 = parts[1].parse()?;

        // Establish TCP connection with the peer
        let mut stream = TcpStream::connect((peer_ip, peer_port)).await?;
        let torrent = Torrent::from_file(path)?;
        let info_hash = torrent.info_hash()?;

        let mut handshake = vec![19]; // length of protocol string
        handshake.extend(b"BitTorrent protocol"); // protocol string
        handshake.extend(&[0; 8]); // reserved bytes
        handshake.extend(&info_hash);
        handshake.extend(b"00112233445566778899");
        stream.write_all(&handshake).await?;

        let mut response = vec![0; 68]; // length of handshake message
        stream.read_exact(&mut response).await?;

        let peer_id = &response[48..68];
        let peer_id_hex = hex::encode(peer_id);

        println!("Peer ID: {}", peer_id_hex);

        Ok(())
    }

    pub async fn download_piece(output: &str, path: &str, piece_index: &usize) -> Result<()>{
        println!("Download piece: {} {} {}", output, path, piece_index);

        Ok(())
    }
}
