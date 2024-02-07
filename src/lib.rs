use chrono::{LocalResult, TimeZone, Utc};
use std::error::Error;
use yahoo_finance_api as yahoo;

pub struct DailyQuote {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
    pub close: f64,
    pub adjclose: f64,
    pub is_volatile: bool,
}

pub struct StockAnalysis {
    pub min_price: f64,
    pub max_price: f64,
    pub min_date: String,
    pub max_date: String,
    pub quotes: Vec<DailyQuote>,
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
            quotes: Vec::new(),
        };

        for quote in quotes {
            if let LocalResult::Single(date) = Utc.timestamp_opt(quote.timestamp as i64, 0) {
                let formatted_date = date.format("%Y-%m-%d").to_string();
                let volatility = determine_volatility(quote.high, quote.low);

                let daily_quote = DailyQuote {
                    date: formatted_date.clone(),
                    open: quote.open,
                    high: quote.high,
                    low: quote.low,
                    volume: quote.volume,
                    close: quote.close,
                    adjclose: quote.adjclose,
                    is_volatile: volatility,
                };
                analysis.quotes.push(daily_quote);

                update_min_max_prices(&mut analysis, quote.close, &formatted_date);
            } else {
                eprintln!("Invalid timestamp for quote");
            }
        }

        Ok(analysis)
    }
}

fn determine_volatility(high: f64, low: f64) -> bool {
    (high - low) / low > 0.02
}

fn update_min_max_prices(analysis: &mut StockAnalysis, price: f64, date: &str) {
    if price < analysis.min_price {
        analysis.min_price = price;
        analysis.min_date = date.to_string();
    }
    if price > analysis.max_price {
        analysis.max_price = price;
        analysis.max_date = date.to_string();
    }
}
