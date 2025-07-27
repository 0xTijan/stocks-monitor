use parser_core::ast::*;
use crate::response::{TrackedItem, Item, ItemPrice};
use std::collections::HashMap;
use crate::types::{Stock, Index, DailyPrice, IndexValue};
use chrono::NaiveDate;
use crate::apis::{fetch_api_data_async, ApiResponse};


#[derive(Debug)]
pub struct EvalContext {
    // === Raw Market Data ===
    pub stocks: HashMap<String, Stock>, 
    pub indexes: HashMap<String, Index>,

    // === Time Series ===
    pub price_series: HashMap<String, Vec<DailyPrice>>,
    pub index_series: HashMap<String, Vec<IndexValue>>,

    // === Derived Series ===
    pub derived_series: HashMap<String, Vec<f64>>,

    /*// === Function Registry ===
    functions: HashMap<String, fn(&EvalContext, Vec<String>) -> Vec<f64>>,*/

    // === Metadata / Settings ===
    pub date_range: (String, String),
    pub tracked_items: Vec<TrackedItem>,
}


impl EvalContext {
    pub fn date_range_len(&self) -> usize {
        let start = NaiveDate::parse_from_str(&self.date_range.0, "%Y-%m-%d")
            .expect("Invalid start date format");
        let end = NaiveDate::parse_from_str(&self.date_range.1, "%Y-%m-%d")
            .expect("Invalid end date format");
        (end - start).num_days().try_into().unwrap()
    }

    pub async fn get_item_prices(&mut self, item_id: &str) -> Option<Vec<f64>> {
        if let Some(ctx_prices) = self.derived_series.get(item_id) {
            return Some(ctx_prices.clone());
        }

        println!("Fetching prices for item: {}, {}", &self.date_range.0, &self.date_range.1);

        // Attempt to fetch
        match fetch_api_data_async(item_id, &self.date_range.0, &self.date_range.1).await {
            Ok(api_response) => {
                match api_response {
                    ApiResponse::Stock(stock_res) => {
                        self.stocks.insert(item_id.to_string(), stock_res.info);
                        self.price_series.insert(item_id.to_string(), stock_res.prices.clone());
                        let prices: Vec<f64> = stock_res
                            .prices
                            .iter()
                            .filter_map(|p| p.last_price)
                            .collect();
                        let volumes: Vec<f64> = stock_res
                            .prices
                            .iter()
                            .filter_map(|p| p.volume)
                            .map(|v| v as f64)
                            .collect();
                        self.derived_series.insert(item_id.to_string(), prices.clone());
                        self.derived_series.insert(format!("{}_volume", item_id), volumes);
                        Some(prices)
                    },
                    ApiResponse::Index(index_res) => {
                        self.indexes.insert(item_id.to_string(), index_res.info);
                        self.index_series.insert(item_id.to_string(), index_res.prices.clone());
                        let prices: Vec<f64> = index_res
                            .prices
                            .iter()
                            .filter_map(|p| p.last_value)
                            .collect();
                        self.derived_series.insert(item_id.to_string(), prices.clone());
                        Some(prices)
                    },
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch API data for {}: {}", item_id, err);
                None
            }
        }
    }

    pub fn get_item_data(&mut self, item_id: &str) -> Option<Item> {
        if let Some(stock) = self.stocks.get(item_id) {
            return Some(Item::Stock(stock.clone()));
        }
        if let Some(index) = self.indexes.get(item_id) {
            return Some(Item::Index(index.clone()));
        }
        None
        
        // get id from symbol
        // fetch prices and item info and store in context
        // return the data
    }
}