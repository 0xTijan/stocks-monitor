// definition of response struct
use serde::{Serialize, Deserialize};
use crate::types::*;
use std::collections::HashMap;


// MAIN RESPONSE STRUCT
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub plot: Option<PlotResponse>,
    pub filter: Option<FilterResponse>,
    pub sort: Option<SortResponse>,
    pub backtest: Option<BacktestResponse>,
}



// PLOT RESPONSE
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlotResponse {
    pub charts: Vec<Chart>,
}

// FILTER RESPONSE
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterResponse {
    pub items: Vec<FilterSortItem>,
}

// SORT RESPONSE
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortResponse {
    pub items: Vec<FilterSortItem>,
}

// BACKTEST RESPONSE
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacktestResponse {
    pub backtests: Vec<Backtest>,
}



// TYPES
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backtest {
    pub item: Item,
    pub trades: Vec<Trade>,
    pub performance: Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Performance {
    pub total_trades: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
    pub total_profit: f64,
    pub total_loss: f64,
    pub profit_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterSortItem {
    pub item: Item,
    pub extra_data: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub item: Item,
    pub data: Vec<ChartData>,
    pub chart_type: ChartType,
    pub panel_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum ChartType {
    Volume,
    Price,
    Indicator,
    IndicatorPanel
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Item {
    Stock(Stock),
    Index(Index),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartData {
    pub date: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedItem {
    pub id: String,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Stock,
    Index,
    Derived,
}

// all extra data fields contain values of indicators, functions, or other metrics that were needed to generate the response