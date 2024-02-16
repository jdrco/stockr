mod utils;

mod plot;

use crate::plot::plot_stock_quotes;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// Define the structure matching the JSON for deserialization
#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
    adjclose: f64,
    is_volatile: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    min_close_price: f64,
    max_close_price: f64,
    min_close_date: String,
    max_close_date: String,
    start_date: String,
    end_date: String,
    min_low_price: f64,
    max_high_price: f64,
    regular_quotes: Vec<Quote>,
    volatile_quotes: Vec<Quote>,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, stockr!");
}

#[wasm_bindgen]
pub struct Chart {}

#[wasm_bindgen]
impl Chart {
    pub async fn fetch_stock_data(symbol: String) -> Result<JsValue, JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let url = format!("http://127.0.0.1:8080/stock/{}", symbol);

        let request = Request::new_with_str_and_init(&url, &opts)?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;

        // Deserialize
        let market_data: MarketData =
            from_value(json.clone()).map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Convert dates from String to NaiveDate
        let start_date = NaiveDate::parse_from_str(&market_data.start_date, "%Y-%m-%d")
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let end_date = NaiveDate::parse_from_str(&market_data.end_date, "%Y-%m-%d")
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Convert quotes to the specified tuple format
        let regular_quotes: Vec<(NaiveDate, f64, f64, f64, f64)> = market_data
            .regular_quotes
            .iter()
            .map(|q| {
                (
                    NaiveDate::parse_from_str(&q.date, "%Y-%m-%d").unwrap(),
                    q.open,
                    q.high,
                    q.low,
                    q.close,
                )
            })
            .collect();

        let volatile_quotes: Vec<(NaiveDate, f64, f64, f64, f64)> = market_data
            .volatile_quotes
            .iter()
            .map(|q| {
                (
                    NaiveDate::parse_from_str(&q.date, "%Y-%m-%d").unwrap(),
                    q.open,
                    q.high,
                    q.low,
                    q.close,
                )
            })
            .collect();

        if let Err(e) = plot_stock_quotes(
            "canvas",
            &regular_quotes,
            &volatile_quotes,
            start_date,
            end_date,
            market_data.min_low_price.clone(),
            market_data.max_high_price.clone(),
            "AAPL",
        ) {
            eprintln!("Error generating plot: {}", e);
        }

        Ok(json)
    }
}
