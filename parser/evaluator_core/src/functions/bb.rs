
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
