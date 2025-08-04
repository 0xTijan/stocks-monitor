use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use gloo_net::http::Request;
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



// WASM IMPLEMENTATION
pub async fn fetch_api_data_async(symbol: &str, from: &str, to: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!("https://monitor-api.tijan.dev/api/general/{}?from={}&to={}", symbol, from, to);

    let text = Request::get(&url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let response: ApiResponse = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(response)
}

pub async fn fetch_all_stocks() -> Result<Vec<Stock>, Box<dyn Error>> {
    let url = "https://monitor-api.tijan.dev/api/stocks";

    let text = Request::get(url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let stocks: Vec<Stock> = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(stocks)
}

pub async fn fetch_all_indexes() -> Result<Vec<Index>, Box<dyn Error>> {
    let url = "https://monitor-api.tijan.dev/api/indexes";

    let text = Request::get(url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let indexes: Vec<Index> = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(indexes)
}

pub async fn fetch_all_indexes_prices(from: &str, to: &str) -> Result<HashMap<String, Vec<IndexValue>>, Box<dyn Error>> {
    let url = format!("https://monitor-api.tijan.dev/api/indexes/prices?from={}&to={}", from, to);

    let text = Request::get(&url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let indexes: HashMap<String, Vec<IndexValue>> = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(indexes)
}

pub async fn fetch_all_stocks_prices(from: &str, to: &str) -> Result<HashMap<String, Vec<DailyPrice>>, Box<dyn Error>> {
    let url = format!("https://monitor-api.tijan.dev/api/stocks/prices?from={}&to={}", from, to);

    let text = Request::get(&url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let stocks: HashMap<String, Vec<DailyPrice>> = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(stocks)
}


/*
// NATIVE IMPLEMENTATION
pub async fn fetch_api_data_async(symbol: &str, from: &str, to: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!("https://monitor-api.tijan.dev/api/general/{}?from={}&to={}", symbol, from, to);

    let text = Request::get(&url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let response: ApiResponse = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(response)
}

pub async fn fetch_all_stocks() -> Result<Vec<Stock>, Box<dyn Error>> {
    let url = "https://monitor-api.tijan.dev/api/stocks";

    let text = Request::get(url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let stocks: Vec<Stock> = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(stocks)
}

pub async fn fetch_all_indexes() -> Result<Vec<Index>, Box<dyn Error>> {
    let url = "https://monitor-api.tijan.dev/api/indexes";

    let text = Request::get(url)
        .send()
        .await
        .map_err(|e| boxed(&format!("Request error: {}", e)))?
        .text()
        .await
        .map_err(|e| boxed(&format!("Read body error: {}", e)))?;

    let indexes: Vec<Index> = serde_json::from_str(&text)
        .map_err(|e| boxed(&format!("JSON error: {}", e)))?;

    Ok(indexes)
}
*/


/// Helper to convert strings into Box<dyn Error>
fn boxed(msg: &str) -> Box<dyn Error> {
    msg.to_string().into()
}
