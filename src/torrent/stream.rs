use super::{HandShake, Torrent, HANDSHAKE_BUF_SIZE};
use anyhow::{anyhow, Context, Result};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};

pub struct Stream {
    pub connection: TcpStream,
}

impl Stream {
    pub async fn connect(peer_address: &str) -> Result<Self> {
        let connection = TcpStream::connect(peer_address).await.context(format!(
            "Ctx: stream connection failed to peer address: {peer_address}"
        ))?;
        Ok(Self { connection })
    }

    pub async fn handshake(&mut self, handshake: HandShake) -> Result<[u8; HANDSHAKE_BUF_SIZE]> {
        self.connection
            .write_all(&handshake.as_bytes())
            .await
            .context("Ctx: write handshake bytes failed")?;

        let mut buf = [0u8; HANDSHAKE_BUF_SIZE];
        self.connection
            .read_exact(&mut buf)
            .await
            .context("Ctx: read handshake bytes failed")?;

        Ok(buf)
    }

    pub async fn bitfield(&mut self) -> Result<()> {
        let length = self.message_length().await?;
        let mut buf = vec![0u8; length as usize];
        self.connection
            .read_exact(&mut buf)
            .await
            .context("Ctx: read bitfield buffer failed")?;

        match MessageType::from_id(buf[0]) {
            Some(MessageType::Bitfield) => Ok(()),
            _ => Err(anyhow!("Expected bitfield")),
        }
    }

    pub async fn interested(&mut self) -> Result<()> {
        let mut interested = [0u8; 5];
        interested[3] = 1;
        interested[4] = MessageType::Interested.id();
        self.connection
            .write_all(&interested)
            .await
            .context("Ctx: write interested buffer failed")?;
        Ok(())
    }

    pub async fn get_piece_data(&mut self, piece: u32, torrent: &Torrent) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        let mut block_index: u32 = 0;
        let mut block_size: u32 = 16 * 1024;

        println!("Debug: info.pieces.len: {}", torrent.info.pieces.len());
        println!("Debug: piece_length: {}", torrent.info.piece_length);

        let mut remaining_bytes: u32 = if piece == (torrent.info.pieces.len() / 20) as u32 - 1 {
            (torrent.info.length as u32) % (torrent.info.piece_length as u32)
        } else {
            torrent.info.piece_length as u32
        };

        while remaining_bytes > 0 {
            if remaining_bytes < block_size {
                block_size = remaining_bytes;
            }

            self.send_request_piece(piece, block_index, block_size)
                .await?;
            let request_buf = self
                .read_request_piece()
                .await
                .context("Ctx: Reading request piece")?;

            let mut piece_data_index = [0u8; 4];
            piece_data_index.copy_from_slice(&request_buf[1..5]);
            let mut piece_offset_begin = [0u8; 4];
            piece_offset_begin.copy_from_slice(&request_buf[5..9]);
            let data_block = request_buf[9..].to_vec();
            data.extend(data_block);
            remaining_bytes -= block_size;
            block_index += block_size;
        }
        Ok(data)
    }

    pub async fn wait_unchoke(&mut self) -> Result<()> {
        let length = self.message_length().await?;
        let mut unchoke_message_buffer = vec![0; length as usize];

        loop {
            self.timeout(&mut unchoke_message_buffer, Duration::from_secs(10))
                .await?;
            if unchoke_message_buffer[0] == MessageType::Unchoke.id() {
                break;
            }
        }
        Ok(())
    }

    async fn send_request_piece(
        &mut self,
        piece: u32,
        block_index: u32,
        block_size: u32,
    ) -> Result<()> {
        let mut request_piece_buf = [0u8; 17];
        request_piece_buf[0..4].copy_from_slice(&13u32.to_be_bytes()); // Message length: 13
        request_piece_buf[4] = MessageType::Request.id();
        request_piece_buf[5..9].copy_from_slice(&piece.to_be_bytes());
        request_piece_buf[9..13].copy_from_slice(&block_index.to_be_bytes());
        request_piece_buf[13..17].copy_from_slice(&block_size.to_be_bytes());
        self.connection
            .write_all(&request_piece_buf)
            .await
            .context("Ctx: send request piece")?;
        Ok(())
    }

    async fn read_request_piece(&mut self) -> Result<Vec<u8>> {
        let length = self.message_length().await?;
        let mut request_buf = vec![0; length as usize];
        self.connection
            .read_exact(&mut request_buf)
            .await
            .context("Ctx: request piece buf")?;

        if request_buf[0] != MessageType::Piece.id() {
            panic!("expected request piece");
        }

        Ok(request_buf)
    }

    async fn message_length(&mut self) -> Result<u32> {
        let mut length_buf = [0u8; 4];
        self.connection
            .read_exact(&mut length_buf)
            .await
            .context("Ctx: read length buffer")?;
        let length = u32::from_be_bytes(length_buf);
        Ok(length)
    }

    async fn timeout(&mut self, buffer: &mut [u8], timeout_duration: Duration) -> Result<()> {
        let _ = timeout(timeout_duration, self.connection.read_exact(buffer))
            .await
            .context("Ctx: read operation timed out")??;
        Ok(())
    }
}

#[derive(Debug)]
pub enum MessageType {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have,
    Bitfield,
    Request,
    Piece,
    Cancel,
}

impl MessageType {
    pub fn id(&self) -> u8 {
        match self {
            MessageType::Choke => 0,
            MessageType::Unchoke => 1,
            MessageType::Interested => 2,
            MessageType::NotInterested => 3,
            MessageType::Have => 4,
            MessageType::Bitfield => 5,
            MessageType::Request => 6,
            MessageType::Piece => 7,
            MessageType::Cancel => 8,
        }
    }

    pub fn from_id(id: u8) -> Option<MessageType> {
        match id {
            0 => Some(MessageType::Choke),
            1 => Some(MessageType::Unchoke),
            2 => Some(MessageType::Interested),
            3 => Some(MessageType::NotInterested),
            4 => Some(MessageType::Have),
            5 => Some(MessageType::Bitfield),
            6 => Some(MessageType::Request),
            7 => Some(MessageType::Piece),
            8 => Some(MessageType::Cancel),
            _ => None,
        }
    }

    pub fn get_write_buffer<F>(&self, get_values: F) -> Vec<u8>
    where
        F: Fn() -> (u32, u32, u32),
    {
        match self {
            MessageType::Interested => {
                let mut buf = [0u8; 5];
                buf[3] = 1;
                buf[4] = MessageType::Interested.id();
                buf.to_vec()
            }
            MessageType::Request => {
                let (piece, block_index, block_size) = get_values();
                let mut buf = [0u8; 17];
                buf[0..4].copy_from_slice(&13u32.to_be_bytes()); // Message length: 13
                buf[4] = MessageType::Request.id(); // Message ID: 6 (request)
                buf[5..9].copy_from_slice(&piece.to_be_bytes()); // Piece index
                buf[9..13].copy_from_slice(&block_index.to_be_bytes()); // Offset
                buf[13..17].copy_from_slice(&block_size.to_be_bytes()); // Length
                buf.to_vec()
            }
            _ => Vec::new(),
        }
    }
}
