use anyhow::{anyhow, Result};
use reqwest::blocking::*;
use serde::{Deserialize, Serialize};

pub const API_URL: &str = "https://127.0.0.1:8080";

#[derive(Deserialize, Debug)]
pub enum Item {
    Paper,
    Plastic,
    Garbage,
}
#[derive(Deserialize)]
pub struct ClassifyResponse {
    #[serde(rename = "type")]
    item_type: Item,
}

pub fn classify(user_id: &str, image: Vec<u8>) -> Result<ClassifyResponse> {
    let resp = Client::new()
        .post(format!("{}/classify?userId={}", API_URL, user_id))
        .header("Content-Type", "image/png")
        .body(image)
        .send()?;

    if !resp.status().is_success() {
        anyhow!("errored with status {}", resp.status());
    }

    let resp_json = resp.json::<ClassifyResponse>()?;
    Ok(resp_json)
}
