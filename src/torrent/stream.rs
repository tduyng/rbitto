use super::{HandShake, Torrent, HANDSHAKE_BUF_SIZE};
use anyhow::{anyhow, Result};
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
        let connection = TcpStream::connect(peer_address).await?;
        Ok(Self { connection })
    }

    pub async fn handshake(&mut self, handshake: HandShake) -> Result<[u8; HANDSHAKE_BUF_SIZE]> {
        self.connection.write_all(&handshake.as_bytes()).await?;

        let mut buf = [0u8; HANDSHAKE_BUF_SIZE];
        self.connection.read_exact(&mut buf).await?;

        Ok(buf)
    }

    pub async fn bitfield(&mut self) -> Result<()> {
        let length = self.message_length().await?;
        let mut buf = vec![0u8; length as usize];
        self.connection.read_exact(&mut buf).await?;

        match MessageType::from_id(buf[0]) {
            Some(MessageType::Bitfield) => Ok(()),
            _ => Err(anyhow!("Expected bitfield")),
        }
    }

    pub async fn interested(&mut self) -> Result<()> {
        let mut interested = [0u8; 5];
        interested[3] = 1;
        interested[4] = MessageType::Interested.id();
        self.connection.write_all(&interested).await?;
        Ok(())
    }

    pub async fn get_piece_data(&mut self, piece: u32, torrent: &Torrent) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        let mut block_index: u32 = 0;
        let mut block_size: u32 = 16 * 1024;
        let mut remaining_bytes: u32 = if piece == torrent.info.pieces.len() as u32 - 1 {
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
            let request_buf = self.read_request_piece().await?;

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
        self.connection.write_all(&request_piece_buf).await?;
        Ok(())
    }

    async fn read_request_piece(&mut self) -> Result<Vec<u8>> {
        let length = self.message_length().await?;
        let mut request_buf = vec![0; length as usize];
        self.connection.read_exact(&mut request_buf).await?;

        if request_buf[0] != MessageType::Piece.id() {
            panic!("expected request piece");
        }

        Ok(request_buf)
    }

    async fn message_length(&mut self) -> Result<u32> {
        let mut length_buf = [0u8; 4];
        self.connection.read_exact(&mut length_buf).await?;
        let length = u32::from_be_bytes(length_buf);
        Ok(length)
    }

    async fn timeout(&mut self, buffer: &mut [u8], timeout_duration: Duration) -> Result<()> {
        let _ = timeout(timeout_duration, self.connection.read_exact(buffer)).await?;
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
}
