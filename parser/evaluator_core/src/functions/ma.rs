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