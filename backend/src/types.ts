export interface Stock {
    isin: string;
    mic: string;
    symbol: string;
    name?: string;
    nace?: string;
    sector_id?: string;
    sector_name?: string;
    first_trading_date?: string;
    quantity?: number;
    description?: string;
    logo_url?: string;
    website_url?: string;
}
  
export interface Index {
    isin: string;
    mic: string;
    symbol: string;
    name?: string;
}
  
export interface IndexMember {
    index_isin: string;
    stock_isin: string;
    weight?: number;
}
  
export interface DailyPrice {
    id: number;
    stock_isin: string;
    date: string;
    trading_model_id?: string;
    open_price?: number;
    high_price?: number;
    low_price?: number;
    last_price?: number;
    vwap_price?: number;
    change_prev_close_percentage?: number;
    num_trades?: number;
    volume?: number;
    turnover?: number;
    price_currency?: string;
    turnover_currency?: string;
}
  
export interface IndexValue {
    id: number;
    index_isin: string;
    date: string;
    open_value?: number;
    high_value?: number;
    low_value?: number;
    last_value?: number;
    change_prev_close_percentage?: number;
    turnover?: number;
}
  