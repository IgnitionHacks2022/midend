use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum Item {
    Red,
    Blue,
    Garbage,
}

impl TryFrom<String> for Item {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Item::Red),
            "blue" => Ok(Item::Blue),
            "garbage" => Ok(Item::Garbage),
            _ => Err(anyhow!("Could not convert from string to Item")),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ClassifyResponse {
    #[serde(rename = "type")]
    pub item_type: String,
    pub audio: String,
}

// unused
#[derive(Serialize)]
pub struct ClassifyRequest {
    pub contents: String,
    #[serde(rename = "bluetoothIDs")]
    pub bluetooth_ids: Vec<String>,
}
