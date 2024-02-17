use chrono::{NaiveDate, NaiveDateTime};
use yahoo_finance_api::Quote;
use serde::Serialize;

// Struct to hold state of stock symbol
pub struct AppState {
    pub user_input: Option<String>,
}

// Struct to hold data for a daily stock quote
#[derive(Serialize)]
pub struct DailyQuote {
    pub date: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
    pub close: f64,
    pub adjclose: f64,
    pub is_volatile: bool,
}

// Struct to hold data for a stock
#[derive(Serialize)]
pub struct StockAnalysis {
    pub min_close_price: f64,
    pub max_close_price: f64,
    pub min_close_date: NaiveDate,
    pub max_close_date: NaiveDate,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub min_low_price: f64,
    pub max_high_price: f64,
    pub regular_quotes: Vec<DailyQuote>,
    pub volatile_quotes: Vec<DailyQuote>,
}

// Conversion function for date
pub fn timestamp_to_local_date(timestamp: i64) -> NaiveDate {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .unwrap()
        .date()
}

// Calculates volatility for the day
pub fn determine_volatility(high: f64, low: f64) -> bool {
    let volatility_thresh: f64 = 0.02;
    (high - low) / low > volatility_thresh
}

// Used to update the min and max prices of a stock
pub fn update_min_max_prices(analysis: &mut StockAnalysis, quote: Quote, date: NaiveDate) {
    if quote.low < analysis.min_low_price {
        analysis.min_low_price = quote.low;
    }
    if quote.high > analysis.max_high_price {
        analysis.max_high_price = quote.high;
    }
    if quote.close < analysis.min_close_price {
        analysis.min_close_price = quote.close;
        analysis.min_close_date = date;
    }
    if quote.close > analysis.max_close_price {
        analysis.max_close_price = quote.close;
        analysis.max_close_date = date;
    }
}
