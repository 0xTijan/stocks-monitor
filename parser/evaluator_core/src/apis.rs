use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
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

pub async fn fetch_all_stocks() -> Result<Vec<Stock>, Box<dyn Error>> {
    let url = "https://monitor-api.tijan.dev/api/stocks";
    let client = Client::new();

    let text = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let stocks: Vec<Stock> = serde_json::from_str(&text)?;
    Ok(stocks)
}

pub async fn fetch_all_indexes() -> Result<Vec<Index>, Box<dyn Error>> {
    let url = "https://monitor-api.tijan.dev/api/indexes";
    let client = Client::new();

    let text = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let indexes: Vec<Index> = serde_json::from_str(&text)?;
    Ok(indexes)
}