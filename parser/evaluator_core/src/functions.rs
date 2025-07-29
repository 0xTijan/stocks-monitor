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

fn rsi(prices: &Vec<f64>, len: u64) -> Vec<f64> {
    let len = len as usize;
    if prices.len() <= len {
        return vec![];
    }

    let mut gains = Vec::with_capacity(prices.len());
    let mut losses = Vec::with_capacity(prices.len());

    // Calculate gains and losses
    for i in 1..prices.len() {
        let diff = prices[i] - prices[i - 1];
        if diff > 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-diff);
        }
    }

    // Compute initial averages
    let mut avg_gain: f64 = gains[..len].iter().sum::<f64>() / len as f64;
    let mut avg_loss: f64 = losses[..len].iter().sum::<f64>() / len as f64;

    let mut rsis = Vec::with_capacity(prices.len() - len);

    // First RSI
    let rs = if avg_loss == 0.0 {
        f64::INFINITY
    } else {
        avg_gain / avg_loss
    };
    rsis.push(100.0 - (100.0 / (1.0 + rs)));

    // Remaining RSIs
    for i in len..gains.len() {
        avg_gain = (avg_gain * (len as f64 - 1.0) + gains[i]) / len as f64;
        avg_loss = (avg_loss * (len as f64 - 1.0) + losses[i]) / len as f64;

        let rs = if avg_loss == 0.0 {
            f64::INFINITY
        } else {
            avg_gain / avg_loss
        };

        let rsi = 100.0 - (100.0 / (1.0 + rs));
        rsis.push(rsi);
    }

    rsis
}

pub fn sma(prices: &Vec<f64>, len: u64) -> Vec<f64> {
    let len = len as usize;
    if prices.len() < len {
        return vec![];
    }

    let mut result = Vec::with_capacity(prices.len() - len + 1);
    let mut sum: f64 = prices[..len].iter().sum();

    result.push(sum / len as f64);

    for i in len..prices.len() {
        sum = sum - prices[i - len] + prices[i];
        result.push(sum / len as f64);
    }

    result
}

pub fn ema(prices: &Vec<f64>, len: u64) -> Vec<f64> {
    let len = len as usize;
    if prices.len() < len {
        return vec![];
    }

    let alpha = 2.0 / (len as f64 + 1.0);
    let mut result = Vec::with_capacity(prices.len() - len + 1);

    // Start EMA with SMA of first `len` prices
    let mut ema_prev = prices[..len].iter().sum::<f64>() / len as f64;
    result.push(ema_prev);

    for i in len..prices.len() {
        let ema = alpha * prices[i] + (1.0 - alpha) * ema_prev;
        result.push(ema);
        ema_prev = ema;
    }

    result
}

pub fn wma(prices: &Vec<f64>, len: u64) -> Vec<f64> {
    let len = len as usize;
    if prices.len() < len {
        return vec![];
    }

    let mut result = Vec::with_capacity(prices.len() - len + 1);
    let denominator = (len * (len + 1) / 2) as f64;

    for i in (len - 1)..prices.len() {
        let mut weighted_sum = 0.0;
        for j in 0..len {
            weighted_sum += prices[i - j] * (len - j) as f64;
        }
        result.push(weighted_sum / denominator);
    }

    result
}
