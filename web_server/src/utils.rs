use crate::StockAnalysis;
use chrono::{NaiveDate, NaiveDateTime};
use yahoo_finance_api::Quote;

pub fn timestamp_to_local_date(timestamp: i64) -> NaiveDate {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .unwrap()
        .date()
}

// TODO: Make sure this is correct calculation
pub fn determine_volatility(high: f64, low: f64) -> bool {
    (high - low) / low > 0.02
}

// TODO: Make sure this is correct calculation
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
