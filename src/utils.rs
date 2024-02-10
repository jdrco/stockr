use crate::analysis::StockAnalysis;
use chrono::{NaiveDate, NaiveDateTime};

pub fn timestamp_to_local_date(timestamp: i64) -> NaiveDate {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .unwrap()
        .date()
}

pub fn determine_volatility(high: f64, low: f64) -> bool {
    (high - low) / low > 0.02
}

pub fn update_min_max_prices(analysis: &mut StockAnalysis, price: f64, date: NaiveDate) {
    if price < analysis.min_price {
        analysis.min_price = price;
        analysis.min_date = date;
    }
    if price > analysis.max_price {
        analysis.max_price = price;
        analysis.max_date = date;
    }
}
