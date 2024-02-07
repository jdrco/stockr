mod chart;
use charming::theme::Theme;
use charming::HtmlRenderer;
use clap::Parser;
use std::path::Path;
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
            println!("Date\t\tOpen\tHigh\tLow\tClose\tVolume\t\tAdj_Close\tVolatile");
            for quote in &analysis.quotes {
                println!(
                    "{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{}\t{:.2}\t\t{}",
                    quote.date,
                    quote.open,
                    quote.high,
                    quote.low,
                    quote.close,
                    quote.volume,
                    quote.adjclose,
                    if quote.is_volatile { "Yes" } else { "No" }
                );
            }
            let chart = chart::chart_from_analysis(&analysis.quotes, &analysis.min_price);
            let mut renderer = HtmlRenderer::new("Chart", 1000, 600).theme(Theme::Dark);
            let file_path = Path::new("stonks.html");
            match renderer.save(&chart, file_path) {
                Ok(_) => println!("Chart saved successfully."),
                Err(e) => {
                    eprintln!("Error saving chart: {:?}", e);
                }
            }
        }
        Err(e) => eprintln!("Error analyzing stock: {}", e),
    }
}
