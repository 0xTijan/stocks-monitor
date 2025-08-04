// definition of response struct
use serde::{Serialize, Deserialize};
use crate::types::*;
use std::collections::HashMap;
use crate::types::{IndexValue, DailyPrice};


// MAIN RESPONSE STRUCT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub charts: Option<Vec<Chart>>,
    pub matching_items: Option<Vec<ResponseItem>>,
    pub backtest: Option<Vec<Backtest>>,
}

// TYPES
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseItem {
    Stock(Stock),
    Index(Index),
    Derived(Derived)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derived {
    pub id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backtest {
    pub item: Item,
    pub trades: Vec<Trade>,
    pub performance: Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub entry_date: String,
    pub exit_date: String,
    pub entry_price: f64,
    pub exit_price: f64,
    pub profit: f64,
    pub profit_percentage: f64,
    pub extra_data: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Performance {
    pub total_trades: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
    pub total_profit: f64,
    pub total_loss: f64,
    pub profit_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSortItem {
    pub item: Item,
    pub extra_data: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub id: String,
    pub data: Vec<ChartData>,
    pub chart_type: ChartType,
    pub panel_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Volume,
    Price,
    Indicator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Stock(Stock),
    Index(Index),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemPrice {
    StockPrice(DailyPrice),
    IndexPrice(IndexValue),
    DerivedPrice(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub date: String,
    pub value: (f64, f64, f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedItem {
    pub id: String,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Stock,
    Index,
    Derived,
}

// all extra data fields contain values of indicators, functions, or other metrics that were needed to generate the response