use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct IndexEntry {
    pub git: String,
    pub author: String
}

pub async fn get_index() -> anyhow::Result<HashMap<String, IndexEntry>> {
    let x = reqwest::get("https://raw.githubusercontent.com/NaKeRNarolino/mosaic_index/refs/heads/main/index.json").await?;

    let t = x.text().await?;

    Ok(serde_json::from_str(&t)?)
}