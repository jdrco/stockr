use stockr::parse_args;
use stockr::plot_stock_quotes;
use stockr::StockMonitor;

#[tokio::main]
async fn main() {
    let args = parse_args();
    let monitor = StockMonitor::new(args.symbol);

    match monitor.analyze_stock().await {
        Ok(analysis) => {
            let reg_quotes_to_plot = analysis.get_regular_quotes_for_plot();
            let vol_quotes_to_plot = analysis.get_volatile_quotes_for_plot();

            if let Err(e) = plot_stock_quotes(
                &reg_quotes_to_plot,
                &vol_quotes_to_plot,
                analysis.start_date,
                analysis.end_date,
                "plots-output",
            ) {
                eprintln!("Error generating plot: {}", e);
            }
            println!("Analysis Report:");
            println!("Min Price: {:.2} on {}", analysis.min_price, analysis.min_date);
            println!("Max Price: {:.2} on {}", analysis.max_price, analysis.max_date);
        }
        Err(e) => eprintln!("Error analyzing stock: {}", e),
    }
}
