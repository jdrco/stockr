use clap::Parser;

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
}
