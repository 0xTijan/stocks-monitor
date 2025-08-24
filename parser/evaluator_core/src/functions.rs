use crate::context::EvalContext;
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

pub fn rsi(prices: &Vec<(String, (f64, f64, f64, f64))>, len: u64) -> Vec<(String, (f64, f64, f64, f64))> {
    let len = len as usize;
    if prices.len() <= len {
        return vec![];
    }

    let values: Vec<f64> = prices.iter().map(|(_, v)| v.0).collect();
    let dates: Vec<String> = prices.iter().map(|(d, _)| d.clone()).collect();

    let mut gains = Vec::with_capacity(values.len());
    let mut losses = Vec::with_capacity(values.len());

    for i in 1..values.len() {
        let diff = values[i] - values[i - 1];
        if diff > 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-diff);
        }
    }

    let mut avg_gain: f64 = gains[..len].iter().sum::<f64>() / len as f64;
    let mut avg_loss: f64 = losses[..len].iter().sum::<f64>() / len as f64;

    let mut rsis: Vec<(String, (f64, f64, f64, f64))> = Vec::with_capacity(values.len() - len);
    let mut date_offset = len;

    let rs = if avg_loss == 0.0 {
        f64::INFINITY
    } else {
        avg_gain / avg_loss
    };
    rsis.push((dates[date_offset].clone(), (100.0 - (100.0 / (1.0 + rs)), 0.0, 0.0, 0.0)));
    date_offset += 1;

    for i in len..gains.len() {
        avg_gain = (avg_gain * (len as f64 - 1.0) + gains[i]) / len as f64;
        avg_loss = (avg_loss * (len as f64 - 1.0) + losses[i]) / len as f64;

        let rs = if avg_loss == 0.0 {
            f64::INFINITY
        } else {
            avg_gain / avg_loss
        };
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        rsis.push((dates[date_offset].clone(), (rsi, 0.0, 0.0, 0.0)));
        date_offset += 1;
    }

    rsis
}

pub fn sma(prices: &Vec<(String, (f64, f64, f64, f64))>, len: u64) -> Vec<(String, (f64, f64, f64, f64))> {
    let len = len as usize;
    if prices.len() < len {
        return vec![];
    }

    let values: Vec<f64> = prices.iter().map(|(_, v)| v.0).collect();
    let dates: Vec<String> = prices.iter().map(|(d, _)| d.clone()).collect();

    let mut result: Vec<(String, (f64, f64, f64, f64))> = Vec::with_capacity(values.len() - len + 1);
    let mut sum: f64 = values[..len].iter().sum();
    result.push((dates[len - 1].clone(), (sum / len as f64, 0.0, 0.0, 0.0)));

    for i in len..values.len() {
        sum = sum - values[i - len] + values[i];
        result.push((dates[i].clone(), (sum / len as f64, 0.0, 0.0, 0.0)));
    }

    result
}

pub fn ema(prices: &Vec<(String, (f64, f64, f64, f64))>, len: u64) -> Vec<(String, (f64, f64, f64, f64))> {
    let len = len as usize;
    if prices.len() < len {
        return vec![];
    }

    let values: Vec<f64> = prices.iter().map(|(_, v)| v.0).collect();
    let dates: Vec<String> = prices.iter().map(|(d, _)| d.clone()).collect();

    let alpha = 2.0 / (len as f64 + 1.0);
    let mut result: Vec<(String, (f64, f64, f64, f64))> = Vec::with_capacity(values.len() - len + 1);

    let mut ema_prev = values[..len].iter().sum::<f64>() / len as f64;
    result.push((dates[len - 1].clone(), (ema_prev, 0.0, 0.0, 0.0)));

    for i in len..values.len() {
        let ema = alpha * values[i] + (1.0 - alpha) * ema_prev;
        result.push((dates[i].clone(), (ema, 0.0, 0.0, 0.0)));
        ema_prev = ema;
    }

    result
}

pub fn wma(prices: &Vec<(String, (f64, f64, f64, f64))>, len: u64) -> Vec<(String, (f64, f64, f64, f64))> {
    let len = len as usize;
    if prices.len() < len {
        return vec![];
    }

    let values: Vec<f64> = prices.iter().map(|(_, v)| v.0).collect();
    let dates: Vec<String> = prices.iter().map(|(d, _)| d.clone()).collect();

    let mut result: Vec<(String, (f64, f64, f64, f64))> = Vec::with_capacity(values.len() - len + 1);
    let denominator = (len * (len + 1) / 2) as f64;

    for i in (len - 1)..values.len() {
        let mut weighted_sum = 0.0;
        for j in 0..len {
            weighted_sum += values[i - j] * (len - j) as f64;
        }
        result.push((dates[i].clone(), (weighted_sum / denominator, 0.0, 0.0, 0.0)));
    }

    result
}
