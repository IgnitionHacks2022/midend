use anyhow::{anyhow, Result};
use reqwest::*;
use serde_json::json;

use crate::models::{ClassifyResponse, Item};

pub const API_URL: &str = "https://ignition.zhehaizhang.com";

pub async fn classify(user_id: &str, image: Vec<u8>) -> Result<ClassifyResponse> {
    let encoded: String = base64::encode(image);

    let resp = Client::new()
        .post(format!("{}/classify/{}", API_URL, user_id))
        .header("Content-Type", "application/json")
        .body(json!({ "contents": encoded }).to_string())
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow!("errored with status {}", resp.status());
    }

    let resp_json = resp.json::<ClassifyResponse>().await?;
    Ok(resp_json)
}
