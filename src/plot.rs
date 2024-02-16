use chrono::{Duration, NaiveDate};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::error::Error;

pub fn plot_stock_quotes(
    canvas_id: &str,
    regular_quotes: &Vec<(NaiveDate, f64, f64, f64, f64)>,
    volatile_quotes: &Vec<(NaiveDate, f64, f64, f64, f64)>,
    start_date: NaiveDate,
    end_date: NaiveDate,
    min_low_price: f64, // For dynamic y axis
    max_high_price: f64,
    symbol: &str,
) -> Result<(), Box<dyn Error>> {
    let root = CanvasBackend::new(canvas_id)
        .expect("cannot find canvas")
        .into_drawing_area();

    // Save to as an image
    // let filepath = format!("{}/stonks.png", dir);
    // let root = BitMapBackend::new(&filepath, (1400, 960)).into_drawing_area();
    root.fill(&WHITE)?;

    // Adjust the date range to include a margin
    let start_date = start_date.pred_opt().unwrap() - Duration::days(1);
    let end_date = end_date.succ_opt().unwrap() + Duration::days(1);

    // Basic chart configuration
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .margin(60)
        .caption(
            &format!("Monitoring {}", symbol),
            ("sans-serif", 50).into_font(),
        )
        .build_cartesian_2d(
            start_date..end_date,
            ((min_low_price) as f32)..((max_high_price) as f32),
        )?;

    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|d| d.format("%b %d").to_string())
        .light_line_style(&WHITE)
        .draw()?;

    // Drawing regular quotes
    chart
        .draw_series(regular_quotes.iter().map(|x| {
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
        }))?
        .label("Regular Quotes")
        .legend(|(x, y)| {
            let (left, right) = (x, x + 20);
            let (top, bottom) = (y - 2, y + 2);
            let fill = Rectangle::new(
                [(left, top), (right, bottom)],
                RGBColor(98, 209, 61).stroke_width(1),
            );
            fill
        });

    // Drawing volatile quotes
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
        .label("Volatile Quotes")
        .legend(|(x, y)| {
            let (left, right) = (x, x + 20);
            let (top, bottom) = (y - 2, y + 2);
            let fill = Rectangle::new(
                [(left, top), (right, bottom)],
                RGBColor(98, 209, 61).filled(),
            );
            fill
        });

    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;

    root.present()?;

    Ok(())
}
