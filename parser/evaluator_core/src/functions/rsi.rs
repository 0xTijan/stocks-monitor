
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