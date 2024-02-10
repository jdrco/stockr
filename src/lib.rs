use chrono::{Date, Local, NaiveDateTime, TimeZone};
use std::error::Error;
use yahoo_finance_api as yahoo;

pub struct DailyQuote {
    pub date: Date<Local>,
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
    pub min_date: Date<Local>,
    pub max_date: Date<Local>,
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
            min_date: Local::today(), // Initialize with today's date
            max_date: Local::today(), // Initialize with today's date
            quotes: Vec::new(),
        };

        for quote in quotes {
            let local_date = timestamp_to_local_date((quote.timestamp * 1000).try_into().unwrap()); // Ensure milliseconds are correctly converted

            let volatility = determine_volatility(quote.high, quote.low);
            let daily_quote = DailyQuote {
                date: local_date,
                open: quote.open,
                high: quote.high,
                low: quote.low,
                volume: quote.volume,
                close: quote.close,
                adjclose: quote.adjclose,
                is_volatile: volatility,
            };
            analysis.quotes.push(daily_quote);

            update_min_max_prices(&mut analysis, quote.close, local_date);
        }

        Ok(analysis)
    }
}

pub fn timestamp_to_local_date(timestamp_millis: i64) -> Date<Local> {
    let naive =
        NaiveDateTime::from_timestamp_opt(timestamp_millis / 1000, 0).expect("Invalid timestamp"); // More robust error handling
    Local.from_utc_datetime(&naive).date()
}

fn determine_volatility(high: f64, low: f64) -> bool {
    (high - low) / low > 0.02
}

fn update_min_max_prices(analysis: &mut StockAnalysis, price: f64, date: Date<Local>) {
    if price < analysis.min_price {
        analysis.min_price = price;
        analysis.min_date = date;
    }
    if price > analysis.max_price {
        analysis.max_price = price;
        analysis.max_date = date;
    }
}
