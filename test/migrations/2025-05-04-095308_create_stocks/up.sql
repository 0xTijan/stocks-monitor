-- Your SQL goes here
CREATE TABLE stocks (
    isin CHAR(12) PRIMARY KEY,
    mic VARCHAR(10) NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    name TEXT,
    CONSTRAINT unique_stock UNIQUE (mic, symbol)
);

CREATE TABLE daily_prices (
    id SERIAL PRIMARY KEY,
    stock_isin CHAR(12) REFERENCES stocks(isin) ON DELETE CASCADE,
    date DATE NOT NULL,
    trading_model_id VARCHAR(10),
    open_price NUMERIC(10, 2),
    high_price NUMERIC(10, 2),
    low_price NUMERIC(10, 2),
    last_price NUMERIC(10, 2),
    vwap_price NUMERIC(15, 8),
    change_prev_close_percentage NUMERIC(6, 2),
    num_trades INTEGER,
    volume NUMERIC(20, 5),
    turnover NUMERIC(20, 2),
    price_currency CHAR(3),
    turnover_currency CHAR(3),
    CONSTRAINT unique_stock_date UNIQUE (stock_isin, date)
);