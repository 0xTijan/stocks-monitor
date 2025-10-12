use crate::context::EvalContext;
use crate::functions::bb::*;
use crate::functions::ma::*;
use crate::functions::rsi::*;
use crate::helpers::{create_function_id};
use parser_core::ast::FunctionArg;
use crate::response_types::ItemType;

pub async fn handle_calculate_function(ctx: &mut EvalContext, args: &Vec<FunctionArg>, name: &String) {
    // get ids that need to be calculated
    // first check if there are ids in args provided
    let mut ids = args.iter().filter_map(|arg| {
        if let FunctionArg::Ident(ident) = arg {
            Some(ident.clone())
        } else {
            None
        }
    }).collect::<Vec<String>>();
    // if no ids provided, use all tracked items
    if ids.is_empty() {
        ids = ctx.tracked_items.iter()
            .filter_map(|item| match item.item_type {
                ItemType::Stock => Some(item.id.clone()),
                ItemType::Index => Some(item.id.clone()),
                ItemType::Derived => Some(item.id.clone()),
            })
            .collect();
    }

    for id in ids {
        // id for function cache
        let key = create_function_id(&name, &args, &id);
        // check if the function is already calculated
        if ctx.derived_series.contains_key(&key) {
            continue;
        } else {
            let prices = ctx.get_item_prices(&id, true).await;
            match prices {
                Some(prices) => {
                    // calculate the function
                    let result = match name.as_str() {
                        "RSI" => {
                            let len: u64 = match args[0] {
                                FunctionArg::Number(n) => n as u64,
                                _ => 14,
                            };
                            rsi(&prices, len)
                        }
                        "RSIMA" => {
                            let rsi_len: u64 = match args[0] {
                                FunctionArg::Number(n) => n as u64,
                                _ => 14,
                            };
                            let ma_len =  match args[1] {
                                FunctionArg::Number(n) => n as u64,
                                _ => 14,
                            };
                            let rsi = rsi(&prices, rsi_len);
                            sma(&rsi, ma_len)
                        }
                        "MA" => {
                            let len: u64 = match args[0] {
                                FunctionArg::Number(n) => n as u64,
                                _ => 14,
                            };
                            sma(&prices, len)
                        }
                        "EMA" => {
                            let len: u64 = match args[0] {
                                FunctionArg::Number(n) => n as u64,
                                _ => 14,
                            };
                            ema(&prices, len)
                        }
                        "WMA" => {
                            let len: u64 = match args[0] {
                                FunctionArg::Number(n) => n as u64,
                                _ => 14,
                            };
                            wma(&prices, len)
                        }
                        "BBWP" => {
                            let bbw_len: usize = match args[0] {
                                FunctionArg::Number(n) => n as usize,
                                _ => 13,
                            };
                            let lookback: usize = match args[1] {
                                FunctionArg::Number(n) => n as usize,
                                _ => 252,
                            };
                            bbwp(&prices, bbw_len, lookback)
                        }
                        "BBWPMA" => {
                            let bbw_len: usize = match args[0] {
                                FunctionArg::Number(n) => n as usize,
                                _ => 13,
                            };
                            let lookback: usize = match args[1] {
                                FunctionArg::Number(n) => n as usize,
                                _ => 252,
                            };
                            let ma_len =  match args[1] {
                                FunctionArg::Number(n) => n as u64,
                                _ => 14,
                            };
                            let bbwp = bbwp(&prices, bbw_len, lookback);
                            sma(&bbwp, ma_len)
                        }
                        _ => {
                            println!("Unknown function: {}", name);
                            continue;
                        }
                    };

                    // store the result in the context
                    ctx.derived_series.insert(key, result);
                }
                None => panic!("No prices found for item: {}", id),
            }
        }
    }
}

pub fn indicator_to_panel_id(string: &str) -> i32 {
    match string.split('_').next().unwrap_or("") {
        "RSI" | "RSIMA" | "BBWP" | "BBWPMA" => 1,   // list of indicators that range from 0 to 100 (are in separate panel)
        _ => 0,                 // default panel for other indicators (all lines that go on candles)
    }
}
