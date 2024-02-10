use crate::utils::{determine_volatility, timestamp_to_local_date, update_min_max_prices};
use chrono::NaiveDate;
use std::error::Error;
use yahoo_finance_api as yahoo;

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

pub struct StockAnalysis {
    pub min_price: f64,
    pub max_price: f64,
    pub min_date: NaiveDate,
    pub max_date: NaiveDate,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub regular_quotes: Vec<DailyQuote>,
    pub volatile_quotes: Vec<DailyQuote>,
}

impl StockAnalysis {
    fn log_and_prepare_quotes_for_plot(
        quotes: &[DailyQuote],
    ) -> Vec<(NaiveDate, f64, f64, f64, f64)> {
        println!("Date\t\t\tOpen\tHigh\tLow\tClose");
        for quote in quotes {
            println!(
                "{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",
                quote.date, quote.open, quote.high, quote.low, quote.close,
            );
        }
        quotes
            .iter()
            .map(|quote| (quote.date, quote.open, quote.high, quote.low, quote.close))
            .collect()
    }

    pub fn get_regular_quotes_for_plot(&self) -> Vec<(NaiveDate, f64, f64, f64, f64)> {
        println!("Regular Quotes");
        Self::log_and_prepare_quotes_for_plot(&self.regular_quotes)
    }

    pub fn get_volatile_quotes_for_plot(&self) -> Vec<(NaiveDate, f64, f64, f64, f64)> {
        println!("Volatile Quotes");
        Self::log_and_prepare_quotes_for_plot(&self.volatile_quotes)
    }
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
            min_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            max_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            start_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            regular_quotes: Vec::new(),
            volatile_quotes: Vec::new(),
        };

        if let Some(first_quote) = quotes.first() {
            analysis.start_date =
                timestamp_to_local_date(first_quote.timestamp.try_into().unwrap());
        }
        if let Some(last_quote) = quotes.last() {
            analysis.end_date = timestamp_to_local_date(last_quote.timestamp.try_into().unwrap());
        }

        for quote in quotes {
            let local_date = timestamp_to_local_date(quote.timestamp.try_into().unwrap());

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
