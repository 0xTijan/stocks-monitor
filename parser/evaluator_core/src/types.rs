// definition of response struct
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stock {
    pub isin: String,
    pub mic: String,
    pub symbol: String,
    pub name: Option<String>,
    pub nace: Option<String>,
    pub sector_id: Option<String>,
    pub sector_name: Option<String>,
    pub first_trading_date: Option<String>,
    pub quantity: Option<u64>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
    pub last_price: Option<f64>,
    pub change_prev_close_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub isin: String,
    pub mic: String,
    pub symbol: String,
    pub name: Option<String>,
    pub last_value: Option<f64>,
    pub change_prev_close_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexMember {
    pub index_isin: String,
    pub stock_isin: String,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyPrice {
    pub id: u64,
    pub stock_isin: String,
    pub date: String,
    pub trading_model_id: Option<String>,
    pub open_price: Option<f64>,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    pub last_price: Option<f64>,
    pub vwap_price: Option<f64>,
    pub change_prev_close_percentage: Option<f64>,
    pub num_trades: Option<u64>,
    pub volume: Option<u64>,
    pub turnover: Option<f64>,
    pub price_currency: Option<String>,
    pub turnover_currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexValue {
    pub id: u64,
    pub index_isin: String,
    pub date: String,
    pub open_value: Option<f64>,
    pub high_value: Option<f64>,
    pub low_value: Option<f64>,
    pub last_value: Option<f64>,
    pub change_prev_close_percentage: Option<f64>,
    pub turnover: Option<f64>,
}
