use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// The stock symbol to monitor
    #[clap(short, long)]
    pub symbol: String,
}

pub fn parse_args() -> Args {
    // TODO: need better error handling for breaking inputs
    let mut args: Args = Args::parse();
    args.symbol = args.symbol.to_uppercase();
    args
}
