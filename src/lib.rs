mod cli;
mod plot;
mod analysis;
mod utils;

pub use cli::parse_args;
pub use plot::plot_stock_quotes;
pub use analysis::{DailyQuote, StockAnalysis, StockMonitor};