use crate::torrent::Torrent;
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Network {}

impl Network {
    pub async fn handshake(path: &str, peer_ip: &str, peer_port: u16) -> Result<TcpStream> {
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

        Ok(stream)
    }
}
