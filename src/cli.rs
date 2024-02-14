use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
pub struct Args {
    /// The stock symbol to monitor
    #[clap(short, long)]
    pub symbol: String,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let symbol_arg_present = args.iter().any(|arg| arg == "--symbol");

    if (args.len() < 3) || (!symbol_arg_present) {
        println!("Required arguments were not provided, please follow the given usage: \n
        USAGE: cargo run -- --symbol <STOCKSYMBOL> \n
        or use '--help' to ");
        println!("Please Enter the Symbol or 'exit' to end program: ");

        let mut input_symbol = String::new();
        loop {
            std::io::stdin().read_line(&mut input_symbol).expect("Failed to read line");
            let trimmed = input_symbol.trim();

            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Exiting program.");
                std::process::exit(0);
            } else if !trimmed.is_empty() {
                return Args::parse_from(vec!["stockr", "--symbol", trimmed]);
            } else {
                println!("Invalid input. Please enter a valid symbol or type 'exit' to quit");
                input_symbol.clear();
            }
        }
    } else {
        Args::parse()
    }
}
