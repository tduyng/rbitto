use crate::utils;
use anyhow::Result;

pub struct Commands {}

impl Commands {
    pub fn decode(data: &str) -> Result<()> {
        let decoded_value = utils::decode(data);

        println!("{}", decoded_value);
        Ok(())
    }
}
