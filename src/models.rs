use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum Item {
    Paper,
    Plastic,
    Garbage,
}

#[derive(Deserialize)]
pub struct ClassifyResponse {
    #[serde(rename = "type")]
    pub item_type: Item,
}
