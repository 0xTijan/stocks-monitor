use crate::response::{Item, ItemPrice};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::runtime::Runtime;
use crate::types::{Index, Stock, DailyPrice, IndexValue};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ApiResponse {
    Stock(StockResponse),
    Index(IndexResponse),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StockResponse {
    pub info: Stock,
    pub prices: Vec<DailyPrice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexResponse {
    pub info: Index,
    pub prices: Vec<IndexValue>,
}

pub async fn fetch_api_data_async(symbol: &str, from: &str, to: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!("https://monitor-api.tijan.dev/api/general/{}?from={}&to={}", symbol, from, to);
    let client = Client::new();

    let text = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let response: ApiResponse = serde_json::from_str(&text)?;
    Ok(response)
}
