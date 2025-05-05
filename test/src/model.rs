use diese::Queryable;

#[derive(Queryable, Debug)]
pub struct Stock {
    pub isin: String,
    pub mic: String,
    pub symbol: String,
    pub name: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = "stocks")]
pub struct NewStock<'a> {
    pub isin: &'a String,
    pub mic: &'a String,
    pub symbol: &'a String,
    pub name: &'a Option<String>,
}

#[derive(Queryable, Debug)]
pub struct NewDailyPrice {
    pub id: i64,
    pub stock_isin: String,
    pub date: String,
    pub trading_model_id: String,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub last_price: f64,
    pub vwap_price: f64,
    pub change_prev_close_percentage: f64,
    pub num_trades: i32,
    pub volume: f64,
    pub turnover: f64,
    pub price_currency: String,
    pub turnover_currency: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = "daily_prices")]
pub struct NewDailyPrice<'a> {
    pub stock_isin: &'a String,
    pub date: &'a String,
    pub trading_model_id: &'a String,
    pub open_price: &'a f64,
    pub high_price: &'a f64,
    pub low_price: &'a f64,
    pub last_price: &'a f64,
    pub vwap_price: &'a f64,
    pub change_prev_close_percentage: &'a f64,
    pub num_trades: &'a i32,
    pub volume: &'a f64,
    pub turnover: &'a f64,
    pub price_currency: &'a String,
    pub turnover_currency: &'a String,
}
