use chrono::{Date, Duration, Local};
use clap::Parser;
use plotters::prelude::*;
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
    // TODO: Need to perform better error handling for inputs
    let monitor = StockMonitor::new(args.symbol);
    match monitor.analyze_stock().await {
        Ok(analysis) => {
            let reg_quotes_to_plot: Vec<(Date<Local>, f64, f64, f64, f64)> = analysis
                .regular_quotes
                .into_iter()
                .map(|quote| (quote.date, quote.open, quote.high, quote.low, quote.close))
                .collect();
            println!("Regular Quotes");
            println!("Date\t\t\tOpen\tHigh\tLow\tClose");
            for quote in &reg_quotes_to_plot {
                println!(
                    "{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",
                    quote.0, quote.1, quote.2, quote.3, quote.4,
                );
            }
            let vol_quotes_to_plot: Vec<(Date<Local>, f64, f64, f64, f64)> = analysis
                .volatile_quotes
                .into_iter()
                .map(|quote| (quote.date, quote.open, quote.high, quote.low, quote.close))
                .collect();
            println!("Volatile Quotes");
            println!("Date\t\t\tOpen\tHigh\tLow\tClose");
            for quote in &vol_quotes_to_plot {
                println!(
                    "{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",
                    quote.0, quote.1, quote.2, quote.3, quote.4,
                );
            }

            let dir = "plots-output";
            let filepath = format!("{}/stonks.png", &dir);
            let root = BitMapBackend::new(&filepath, (1400, 960)).into_drawing_area();
            root.fill(&WHITE).expect("Error filling background.");

            // Get date range
            let (start_date, end_date) = (
                analysis.start_date - Duration::days(1),
                analysis.end_date + Duration::days(1),
            );
            // Basic chart configuration
            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(60)
                .y_label_area_size(60)
                .margin(60)
                .caption("Stonks", ("sans-serif", 50.0).into_font())
                // TODO: The y-axis max and min are hardcoded
                .build_cartesian_2d(start_date..end_date, 160f32..200f32)
                .unwrap();
            chart
                .configure_mesh()
                .light_line_style(&WHITE)
                .draw()
                .unwrap();
            root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));
            chart
                .draw_series(reg_quotes_to_plot.iter().map(|x| {
                    CandleStick::new(
                        x.0,
                        x.1 as f32,
                        x.2 as f32,
                        x.3 as f32,
                        x.4 as f32,
                        RGBColor(98, 209, 61),
                        RGBColor(209, 61, 61),
                        5,
                    )
                }))
                .unwrap();
            chart
                .draw_series(vol_quotes_to_plot.iter().map(|x| {
                    CandleStick::new(
                        x.0,
                        x.1 as f32,
                        x.2 as f32,
                        x.3 as f32,
                        x.4 as f32,
                        RGBColor(98, 209, 61).filled(),
                        RGBColor(209, 61, 61).filled(),
                        5,
                    )
                }))
                .unwrap();
            println!("Plot has been saved to {}", &filepath);
        }
        Err(e) => eprintln!("Error analyzing stock: {}", e),
    }
}
