mod cli;
mod plot;

use crate::cli::parse_args;
use crate::plot::plot_stock_quotes;
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
        }
        Err(e) => eprintln!("Error analyzing stock: {}", e),
    }
}
