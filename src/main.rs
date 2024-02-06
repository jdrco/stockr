use chrono::{LocalResult, TimeZone, Utc};
use clap::Parser;
use yahoo_finance_api as yahoo;

/// Stock Monitor application
#[derive(Parser, Debug)]
struct Args {
    /// The stock symbol to monitor
    #[clap(short, long)]
    symbol: String,
}

fn main() {
    let args = Args::parse();
    println!("Monitoring stock: {}", args.symbol);

    let provider = yahoo::YahooConnector::new();
    let response =
        tokio_test::block_on(provider.get_quote_range(&args.symbol, "1d", "6mo")).unwrap();
    let quotes = response.quotes().unwrap();

    let mut min_price = std::f64::MAX;
    let mut max_price = std::f64::MIN;
    let mut min_date = String::new();
    let mut max_date = String::new();
    let mut chart_data = Vec::new();
    let mut volatile_days = Vec::new();

    for quote in quotes {
        let date_opt = Utc.timestamp_opt(quote.timestamp as i64, 0);
        match date_opt {
            LocalResult::Single(date) => {
                let formatted_date = date.format("%Y-%m-%d").to_string();
                let volatility = (quote.high - quote.low) / quote.low > 0.02;

                if volatility {
                    volatile_days.push(formatted_date.clone());
                }

                chart_data.push((formatted_date.clone(), quote.close));

                if quote.close < min_price {
                    min_price = quote.close;
                    min_date = formatted_date.clone();
                }
                if quote.close > max_price {
                    max_price = quote.close;
                    max_date = formatted_date;
                }
            }
            _ => eprintln!("Invalid timestamp for quote"),
        }
    }

    println!("Date\t\tClosing Price\tVolatile");
    for (date, price) in chart_data {
        let is_volatile = volatile_days.contains(&date);
        println!(
            "{}\t{:.2}\t\t{}",
            date,
            price,
            if is_volatile { "Yes" } else { "No" }
        );
    }

    println!("\nMinimum closing price: {:.2} on {}", min_price, min_date);
    println!("Maximum closing price: {:.2} on {}", max_price, max_date);
}
