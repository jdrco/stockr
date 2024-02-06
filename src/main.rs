use clap::Parser;
use stockr::StockMonitor;

#[derive(Parser, Debug)]
struct Args {
    /// The stock symbol to monitor
    #[clap(short, long)]
    symbol: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let monitor = StockMonitor::new(args.symbol);
    match monitor.analyze_stock().await {
        Ok(analysis) => {
            println!("Date\t\tClosing Price\tVolatile");
            for (date, price) in analysis.chart_data {
                let is_volatile = analysis.volatile_days.contains(&date);
                println!(
                    "{}\t{:.2}\t\t{}",
                    date,
                    price,
                    if is_volatile { "Yes" } else { "No" }
                );
            }

            println!("\nMinimum closing price: {:.2} on {}", analysis.min_price, analysis.min_date);
            println!("Maximum closing price: {:.2} on {}", analysis.max_price, analysis.max_date);
        },
        Err(e) => eprintln!("Error analyzing stock: {}", e),
    }
}
