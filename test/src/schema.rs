// @generated automatically by Diesel CLI.

diesel::table! {
    daily_prices (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 12]
        stock_isin -> Nullable<Char>,
        date -> Date,
        #[max_length = 10]
        trading_model_id -> Nullable<Varchar>,
        open_price -> Nullable<Decimal>,
        high_price -> Nullable<Decimal>,
        low_price -> Nullable<Decimal>,
        last_price -> Nullable<Decimal>,
        vwap_price -> Nullable<Decimal>,
        change_prev_close_percentage -> Nullable<Decimal>,
        num_trades -> Nullable<Integer>,
        volume -> Nullable<Decimal>,
        turnover -> Nullable<Decimal>,
        #[max_length = 3]
        price_currency -> Nullable<Char>,
        #[max_length = 3]
        turnover_currency -> Nullable<Char>,
    }
}

diesel::table! {
    stocks (isin) {
        #[max_length = 12]
        isin -> Char,
        #[max_length = 10]
        mic -> Varchar,
        #[max_length = 10]
        symbol -> Varchar,
        name -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    daily_prices,
    stocks,
);
