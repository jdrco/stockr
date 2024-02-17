use chrono::{Duration, NaiveDate};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::error::Error;

// Plots stock quotes using candlestick charts for regular and volatile days.
pub fn plot_stock_quotes(
    canvas_id: &str,
    regular_quotes: &Vec<(NaiveDate, f64, f64, f64, f64)>,
    volatile_quotes: &Vec<(NaiveDate, f64, f64, f64, f64)>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    min_low_price: f64,  // Defines the lower bound of the y-axis dynamically.
    max_high_price: f64, // Defines the upper bound of the y-axis dynamically.
    symbol: &str,
) -> Result<(), Box<dyn Error>> {
    let root = CanvasBackend::new(canvas_id)
        .expect("cannot find canvas")
        .into_drawing_area();
    root.fill(&WHITE)?;

    // Extend date range slightly for margin.
    let start_date = start_date.pred_opt().unwrap() - Duration::days(1);
    let end_date = end_date.succ_opt().unwrap() + Duration::days(1);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .margin(60)
        .caption(
            &format!("Monitoring {} (Past 6 Months)", symbol),
            ("sans-serif", 40).into_font(),
        )
        .build_cartesian_2d(
            start_date..end_date,
            min_low_price as f32..max_high_price as f32,
        )?;

    // Configure the chart's appearance and axis labels.
    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|d| d.format("%b %d").to_string())
        .light_line_style(&WHITE)
        .draw()?;

    // Plot regular quotes with one style of candlestick.
    chart
        .draw_series(regular_quotes.iter().map(|x| {
            CandleStick::new(
                x.0,
                x.1 as f32,
                x.2 as f32,
                x.3 as f32,
                x.4 as f32,
                RGBColor(98, 209, 61), // Color for the bullish days.
                RGBColor(209, 61, 61), // Color for the bearish days.
                5,                     // Candlestick width.
            )
        }))?
        .label("Empty Candlestick: Regular Quotes")
        .legend(|(x, y)| {
            Rectangle::new(
                [(x, y - 3), (x + 20, y + 3)],
                RGBColor(0, 0, 0).stroke_width(1), // Legend marker.
            )
        });

    // Plot volatile quotes differently to highlight them.
    chart
        .draw_series(volatile_quotes.iter().map(|x| {
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
        }))?
        .label("Filled Candlestick: Volatile Quotes")
        .legend(|(x, y)| {
            Rectangle::new(
                [(x, y - 3), (x + 20, y + 3)],
                RGBColor(0, 0, 0).filled(), // Different legend marker for volatile days.
            )
        });

    // Finalize the chart with a series labels configuration.
    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight) // Positioning of the legend.
        .draw()?;

    root.present()?;

    Ok(())
}
