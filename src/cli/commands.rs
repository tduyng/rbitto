use super::decode_bencoded_value;
use anyhow::Result;

pub struct Commands {}

impl Commands {
    pub fn decode(data: &str) -> Result<()> {
        let decoded_value = decode_bencoded_value(data);

        println!("{}", decoded_value);
        Ok(())
    }
}
