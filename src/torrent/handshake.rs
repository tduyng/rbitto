pub const HANDSHAKE_BUF_INDEX_START: usize = 48;
pub const HANDSHAKE_BUF_SIZE: usize = 68;

pub struct HandShake {
    pub length: u8,
    pub protocol: &'static [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: String,
}

impl HandShake {
    pub fn new(info_hash: [u8; 20]) -> Self {
        Self {
            length: 19,
            protocol: b"BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id: String::from("00112233445566778899"),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.length);
        bytes.extend(self.protocol);
        bytes.extend(self.reserved);
        bytes.extend(self.info_hash);
        bytes.extend(self.peer_id.as_bytes());
        bytes
    }
}
