use crate::{helpers::{enum_to_chart_data, get_today}, response_types::{Chart, ChartType, Derived, Item, ItemType, Response, ResponseItem, TrackedItem}};
use std::collections::{HashMap, HashSet};
use crate::types::{Stock, Index, DailyPrice, IndexValue};
use chrono::NaiveDate;
use crate::apis::*;


#[derive(Debug)]
pub struct EvalContext {
    // === Raw Market Data ===
    pub stocks: HashMap<String, Stock>, 
    pub indexes: HashMap<String, Index>,

    // === Time Series ===
    pub price_series: HashMap<String, Vec<DailyPrice>>,
    pub index_series: HashMap<String, Vec<IndexValue>>,

    // === Derived Series ===
    pub derived_series: HashMap<String, Vec<(String, f64)>>,

    /*// === Function Registry ===
    functions: HashMap<String, fn(&EvalContext, Vec<String>) -> Vec<f64>>,*/

    // === Metadata / Settings ===
    pub date_range: (String, String),
    pub tracked_items: Vec<TrackedItem>,
    pub tracked_ids: HashSet<String>,
}


impl EvalContext {
    pub fn init() -> Self {
        Self {
            stocks: HashMap::new(),
            indexes: HashMap::new(),
            price_series: HashMap::new(),
            index_series: HashMap::new(),
            derived_series: HashMap::new(),
            date_range: ("2024-01-01".to_string(), get_today()),
            tracked_items: Vec::new(),
            tracked_ids: HashSet::new(),
        }
    }

    pub fn date_range_len(&self) -> usize {
        let start = NaiveDate::parse_from_str(&self.date_range.0, "%Y-%m-%d")
            .expect("Invalid start date format");
        let end = NaiveDate::parse_from_str(&self.date_range.1, "%Y-%m-%d")
            .expect("Invalid end date format");
        (end - start).num_days().try_into().unwrap()
    }

    pub async fn get_item_prices(&mut self, item_id: &str, add_to_tracked: bool) -> Option<Vec<(String, f64)>> {
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
                        let prices: Vec<(String, f64)> = stock_res
                            .prices
                            .iter()
                            .filter_map(|p| Some((p.date.clone(), p.last_price.unwrap_or(0.0))))
                            .collect();

                        /*let volumes: Vec<f64> = stock_res
                            .prices
                            .iter()
                            .filter_map(|p| p.volume)
                            .map(|v| v as f64)
                            .collect();*/
                        self.derived_series.insert(item_id.to_string(), prices.clone());
                        //self.derived_series.insert(format!("{}_volume", item_id), volumes);
                        /*if let Some(parent) = parent_id {
                            self.id_to_supporting_ids
                                .entry(parent)
                                .or_insert_with(HashSet::new)
                                .insert(item_id.to_string());
                        } else {*/
                        if add_to_tracked  {    
                            if self.tracked_ids.insert(item_id.to_string()) {
                                self.tracked_items.push(TrackedItem {
                                    id: item_id.to_string(),
                                    item_type: ItemType::Stock,
                                });
                            }
                        }
                        Some(prices)
                    },
                    ApiResponse::Index(index_res) => {
                        self.indexes.insert(item_id.to_string(), index_res.info);
                        self.index_series.insert(item_id.to_string(), index_res.prices.clone());
                        let prices: Vec<(String, f64)> = index_res
                            .prices
                            .iter()
                            .filter_map(|p| Some((p.date.clone(), p.last_value.unwrap_or(0.0))))
                            .collect();

                        self.derived_series.insert(item_id.to_string(), prices.clone());
                        /*if let Some(parent) = parent_id {
                            self.id_to_supporting_ids
                                .entry(parent)
                                .or_insert_with(HashSet::new)
                                .insert(item_id.to_string());
                        } else {*/
                        if add_to_tracked {
                            if self.tracked_ids.insert(item_id.to_string()) {
                                self.tracked_items.push(TrackedItem {
                                    id: item_id.to_string(),
                                    item_type: ItemType::Index,
                                });
                            }
                        }
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

    pub async fn add_all_indexes_to_tracked(&mut self) {
        let res = fetch_all_indexes().await;
        match res {
            Ok(indexes) => {
                for s in indexes {
                    if self.tracked_ids.insert(s.symbol.to_string()) {
                        self.tracked_items.push(TrackedItem {
                            id: s.symbol.to_string(),
                            item_type: ItemType::Index,
                        });
                    }
                    self.indexes.insert(s.symbol.to_string(), s);
                }
            },
            Err(err) => {
                eprintln!("Failed to fetch all indexes: {}", err);
            }
        }
    }

    pub async fn add_all_stocks_to_tracked(&mut self) {
        let res = fetch_all_stocks().await;
        match res {
            Ok(stocks) => {
                for s in stocks {
                    if self.tracked_ids.insert(s.symbol.to_string()) {
                        self.tracked_items.push(TrackedItem {
                            id: s.symbol.to_string(),
                            item_type: ItemType::Stock,
                        });
                    }
                    self.stocks.insert(s.symbol.to_string(), s);
                }
            },
            Err(err) => {
                eprintln!("Failed to fetch all stocks: {}", err);
            }
        }
    }

    pub fn create_response(&mut self, has_plot: bool, has_backtest: bool) -> Response {
        let mut response = Response {
            matching_items: Some(Vec::new()),
            charts: None,
            backtest: None,
        };

        let tracked_items = self.tracked_items.clone();
        for tracked_item in tracked_items {
            // add to matching items
            let data = self.get_item_data(&tracked_item.id);
            match data {
                Some(d) => {
                    let obj: ResponseItem = match d {
                        Item::Index(index) => ResponseItem::Index(index),
                        Item::Stock(stock) => ResponseItem::Stock(stock),
                    };
                    response
                        .matching_items
                        .as_mut()
                        .expect("Expected matching_items to be Some")
                        .push(obj);
                }
                None => {
                    // derived item
                    response
                        .matching_items
                        .as_mut()
                        .expect("Expected matching_items to be Some")
                        .push(ResponseItem::Derived(Derived {id: tracked_item.id}));
                }
            }
        }

        if has_plot {
            // add charts
            if let Some(items) = &response.matching_items {  
                for item in items {
                    // create chart {} for all ids in derived series hashmap that include item id and push to charts
                    let id = match item {
                        ResponseItem::Derived(d) => &d.id,
                        ResponseItem::Index(i) => &i.symbol,
                        ResponseItem::Stock(s) => &s.symbol
                    };

                    let matches = self.get_matching_values_from_derived(id);

                    // add all prices that are in derived
                    for vec in matches {
                        let mut chart_type = ChartType::Price;
                        if id.contains("_") {
                            chart_type = ChartType::Indicator;
                        }
                        let chart_data = enum_to_chart_data(vec.clone());
                        let chart = Chart {
                            id: id.to_string(),
                            chart_type: chart_type,
                            panel_id: 0,
                            data: chart_data
                        };
                        response.charts
                            .get_or_insert_with(Vec::new)
                            .push(chart);
                    }

                    // add volume - if stock - 4 letter id
                    if id.chars().count() == 4 {
                        let volume_data = self.get_volume_for_stock(id);
                        let vol_data = enum_to_chart_data(volume_data);
                        let volume_chart = Chart {
                            id: id.to_string() + " - Volume",
                            chart_type: ChartType::Volume,
                            panel_id: 0,
                            data: vol_data
                        };
                        response.charts
                            .get_or_insert_with(Vec::new)
                            .push(volume_chart);
                    } 
                }
            }
        }

        if has_backtest {
            // add backtest
        }

        response
    }

    pub fn get_matching_values_from_derived<'a>(
        &'a self,
        x: &str,
    ) -> Vec<&'a Vec<(String, f64)>> {
        self.derived_series.iter()
            .filter(|(key, _)| key.contains(x))
            .map(|(_, vec)| vec)
            .collect()
    }

    pub fn get_volume_for_stock(&self, id: &String) -> Vec<(String, f64)> {
        let mut res = Vec::new();
        let prices = self.price_series.get(id);
        if let Some(prices) = prices {
            for price in prices {
                let volume: Option<f64> = price.volume;
                let date = price.date.clone();
                if let Some(vol) = volume {
                    res.push((date, vol as f64));
                }
            }
        }
        res
    }
}