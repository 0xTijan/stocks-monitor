// definition of response struct
use serde::{Serialize, Deserialize};

fn str_to_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.and_then(|v| v.parse::<f64>().ok()))
}

fn str_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.and_then(|v| v.parse::<u64>().ok()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(deserialize_with = "str_to_f64")]
    pub last_price: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub change_prev_close_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub isin: String,
    pub mic: String,
    pub symbol: String,
    pub name: Option<String>,
    #[serde(deserialize_with = "str_to_f64")]
    pub last_value: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub change_prev_close_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMember {
    pub index_isin: String,
    pub stock_isin: String,
    #[serde(deserialize_with = "str_to_f64")]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPrice {
    pub id: u64,
    pub stock_isin: String,
    pub date: String,
    pub trading_model_id: Option<String>,
    #[serde(deserialize_with = "str_to_f64")]
    pub open_price: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub high_price: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub low_price: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub last_price: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub vwap_price: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub change_prev_close_percentage: Option<f64>,
    pub num_trades: Option<u64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub volume: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub turnover: Option<f64>,
    pub price_currency: Option<String>,
    pub turnover_currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexValue {
    pub id: u64,
    pub index_isin: String,
    pub date: String,
    #[serde(deserialize_with = "str_to_f64")]
    pub open_value: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub high_value: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub low_value: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub last_value: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub change_prev_close_percentage: Option<f64>,
    #[serde(deserialize_with = "str_to_f64")]
    pub turnover: Option<f64>,
}

#[derive(Debug)]
pub enum Direction {
    Asc,
    Desc,
}