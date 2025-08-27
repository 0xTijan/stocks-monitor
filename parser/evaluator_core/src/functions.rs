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

// Bollinger Bands (Middle, Upper, Lower, Width)
pub fn bollinger_bands(
    prices: &Vec<(String, (f64, f64, f64, f64))>, 
    period: usize
) -> Vec<(String, (f64, f64, f64, f64))> {
    if prices.len() < period {
        return vec![];
    }

    let closes: Vec<f64> = prices.iter().map(|(_, v)| v.0).collect();
    let dates: Vec<String> = prices.iter().map(|(d, _)| d.clone()).collect();

    let mut bands = Vec::with_capacity(closes.len() - period + 1);

    for i in period - 1..closes.len() {
        let window = &closes[i + 1 - period..=i];
        let sma = window.iter().sum::<f64>() / period as f64;
        let std_dev = calculate_standard_deviation(window, sma);

        let upper = sma + 2.0 * std_dev;
        let lower = sma - 2.0 * std_dev;
        let width = ((upper - lower) / sma) * 100.0;

        bands.push((
            dates[i].clone(),
            (sma, upper, lower, width),
        ));
    }

    bands
}

fn calculate_standard_deviation(window: &[f64], mean: f64) -> f64 {
    let variance = window
        .iter()
        .map(|v| {
            let diff = *v - mean;
            diff * diff
        })
        .sum::<f64>() / window.len() as f64;
    variance.sqrt()
}

/// Rolling standard deviation for one window (population stdev)
fn stddev_window(window: &[f64], mean: f64) -> f64 {
    let var = window.iter().map(|&v| {
        let d = v - mean;
        d * d
    }).sum::<f64>() / window.len() as f64;
    var.sqrt()
}

/// Rolling SMA over `period`. Returns length = N - period + 1.
/// sma[t] corresponds to prices window ending at price index (period - 1 + t).
fn rolling_sma(prices: &[f64], period: usize) -> Vec<f64> {
    if period == 0 || prices.len() < period { return vec![]; }
    let mut out = Vec::with_capacity(prices.len() - period + 1);
    for i in (period - 1)..prices.len() {
        let w = &prices[i + 1 - period ..= i];
        out.push(w.iter().sum::<f64>() / period as f64);
    }
    out
}

/// Rolling stdev using provided SMAs (aligned 1:1 with the windows).
fn rolling_stdev(prices: &[f64], period: usize, smas: &[f64]) -> Vec<f64> {
    if period == 0 || prices.len() < period { return vec![]; }
    let mut out = Vec::with_capacity(smas.len());
    // t indexes SMA/BBW space; price window ends at price index (period - 1 + t)
    for (t, &mean) in smas.iter().enumerate() {
        let end = period - 1 + t;
        let w = &prices[end + 1 - period ..= end];
        out.push(stddev_window(w, mean));
    }
    out
}

/// BBWP: Bollinger Band Width Percentile.
/// Returns (date, (bbwp%, 0, 0, 0)) with length = prices.len() - bbw_len (like your RSI).
pub fn bbwp(
    prices: &Vec<(String, (f64, f64, f64, f64))>,
    bbw_len: usize,
    lookback: usize,
) -> Vec<(String, (f64, f64, f64, f64))> {
    if bbw_len == 0 || prices.len() < bbw_len { return vec![]; }

    let closes: Vec<f64> = prices.iter().map(|(_, v)| v.0).collect();
    let dates: Vec<String> = prices.iter().map(|(d, _)| d.clone()).collect();

    // Basis SMA and stdev over the same windows (BBW index space)
    let basis = rolling_sma(&closes, bbw_len);                  // len_bbw = N - bbw_len + 1
    let dev   = rolling_stdev(&closes, bbw_len, &basis);

    let len_bbw = basis.len();
    if len_bbw == 0 { return vec![]; }

    // BBW = (upper - lower) / basis = 2 * stdev / basis
    let bbw: Vec<f64> = basis.iter().zip(dev.iter())
        .map(|(&b,&d)| if b == 0.0 { 0.0 } else { (2.0 * d) / b })
        .collect();

    let lb = lookback.max(1);

    // We’ll skip the very first BBW sample (t = 0) to align output length to N - bbw_len like RSI.
    // For each t in BBW space, map back to price date at index (bbw_len - 1 + t).
    let mut out = Vec::with_capacity(closes.len() - bbw_len);
    for t in 1..len_bbw {
        let current = bbw[t];
        let window_len = lb.min(t + 1); // how many BBW samples (including current) to consider
        let mut count = 0usize;
        for k in 0..window_len {
            if bbw[t - k] <= current { count += 1; }
        }
        let bbwp_pct = (count as f64 / window_len as f64) * 100.0;

        let date_idx = (bbw_len - 1) + t; // safe: t <= len_bbw-1 → date_idx ≤ N-1
        out.push((dates[date_idx].clone(), (bbwp_pct, 0.0, 0.0, 0.0)));
    }

    out
}
