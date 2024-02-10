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
    pub start_date: Date<Local>,
    pub end_date: Date<Local>,
    pub regular_quotes: Vec<DailyQuote>,
    pub volatile_quotes: Vec<DailyQuote>,
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
            min_date: Local::today(),
            max_date: Local::today(),
            start_date: Local::today(),
            end_date: Local::today(),
            regular_quotes: Vec::new(),
            volatile_quotes: Vec::new(),
        };

        if let Some(first_quote) = quotes.first() {
            analysis.start_date = timestamp_to_local_date((first_quote.timestamp * 1000).try_into().unwrap());
        }
        if let Some(last_quote) = quotes.last() {
            analysis.end_date = timestamp_to_local_date((last_quote.timestamp * 1000).try_into().unwrap());
        }

        for quote in quotes {
            let local_date = timestamp_to_local_date((quote.timestamp * 1000).try_into().unwrap());

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

            if volatility {
                analysis.volatile_quotes.push(daily_quote);
            } else {
                analysis.regular_quotes.push(daily_quote);
            }

            update_min_max_prices(&mut analysis, quote.close, local_date);
        }

        Ok(analysis)
    }
}

pub fn timestamp_to_local_date(timestamp_millis: i64) -> Date<Local> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp_millis / 1000, 0).expect("Invalid timestamp");
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
