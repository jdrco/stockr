
use yahoo_finance_api as yahoo;
use chrono::{LocalResult, Utc, TimeZone};
use std::error::Error;

pub struct StockAnalysis {
    pub min_price: f64,
    pub max_price: f64,
    pub min_date: String,
    pub max_date: String,
    pub chart_data: Vec<(String, f64)>,
    pub volatile_days: Vec<String>,
}

pub struct StockMonitor {
    pub symbol: String,
}

impl StockMonitor {
    pub fn new(symbol: String) -> Self {
        StockMonitor { symbol }
    }

    pub async fn analyze_stock(&self) -> Result<StockAnalysis, Box<dyn Error>> {
        let provider = yahoo::YahooConnector::new();
        let response = provider.get_quote_range(&self.symbol, "1d", "6mo").await?;
        let quotes = response.quotes()?;
        let mut analysis = StockAnalysis {
            min_price: std::f64::MAX,
            max_price: std::f64::MIN,
            min_date: String::new(),
            max_date: String::new(),
            chart_data: Vec::new(),
            volatile_days: Vec::new(),
        };

        for quote in quotes {
            let date_opt = Utc.timestamp_opt(quote.timestamp as i64, 0);
            if let LocalResult::Single(date) = date_opt {
                let formatted_date = date.format("%Y-%m-%d").to_string();
                let volatility = (quote.high - quote.low) / quote.low > 0.02;

                if volatility {
                    analysis.volatile_days.push(formatted_date.clone());
                }

                analysis.chart_data.push((formatted_date.clone(), quote.close));

                if quote.close < analysis.min_price {
                    analysis.min_price = quote.close;
                    analysis.min_date = formatted_date.clone();
                }
                if quote.close > analysis.max_price {
                    analysis.max_price = quote.close;
                    analysis.max_date = formatted_date;
                }
            } else {
                eprintln!("Invalid timestamp for quote");
            }
        }

        Ok(analysis)
    }
}
