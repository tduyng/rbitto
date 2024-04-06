use super::{HandShake, HANDSHAKE_BUF_SIZE};
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Stream {
    pub connection: TcpStream,
}

impl Stream {
    pub async fn connect(peer_address: &str) -> Result<Self> {
        let connection = TcpStream::connect(peer_address).await?;
        Ok(Self { connection })
    }

    pub async fn handshake(&mut self, handshake: HandShake) -> Result<[u8; HANDSHAKE_BUF_SIZE]> {
        self.connection.write_all(&handshake.as_bytes()).await?;

        let mut buf = [0u8; HANDSHAKE_BUF_SIZE];
        self.connection.read_exact(&mut buf).await?;

        Ok(buf)
    }
}
