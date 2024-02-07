use charming::{component::Axis, series::Candlestick, Chart};
use stockr::DailyQuote;

pub fn chart_from_analysis(quotes: &[DailyQuote], min_price: &f64) -> Chart {
    let dates: Vec<String> = quotes.iter().map(|q| q.date.clone()).collect();
    let ohlc_data: Vec<Vec<f64>> = quotes
        .iter()
        .map(|q| vec![q.open, q.close, q.low, q.high])
        .collect();

    // TODO: remove these debug print statements
    println!("Dates Len: {:?}", dates.len());
    println!("OHLC Data Len: {:?}", ohlc_data.len());
    println!("Min price: {:?}", min_price);

    Chart::new()
        .x_axis(Axis::new().data(dates))
        .y_axis(Axis::new().scale(true))
        .series(Candlestick::new().data(ohlc_data))
}

