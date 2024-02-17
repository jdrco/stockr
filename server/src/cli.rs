use clap::Parser;
use std::env;
use regex::Regex;

#[derive(Parser, Debug)]
pub struct Args {
    /// The stock symbol to monitor
    #[clap(short, long)]
    pub symbol: String,

    // The port to run the server on
    #[clap(short, long)]
    pub port: Option<u16>,
}

// Functions to validate input
fn is_alpha(input: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
    re.is_match(input)
}

fn is_valid_symbol(input: &str) -> bool {
    let max_len = 5;
    if is_alpha(input) && input.chars().count() < max_len {
        return true;
    }
    false
}

fn is_numeric(input: &str) -> bool {
    input.chars().all(|c| c.is_digit(10))
}

fn is_valid_port(port: &str) -> bool {
    let max_len: u64 = 65535;
    if is_numeric(port) {
        let result: u64 = port
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .fold(0, |acc, digit| acc * 10 + digit);
        if result <= max_len {
            return true;
        }
    }
    false
}

// parse_args function for collecting and validating inputs
pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let symbol_arg_present = args.iter().any(|arg| arg == "--symbol");
    let port_arg_present = args.iter().any(|arg| arg == "--port");

    if (args.len() < 3) || (!symbol_arg_present && !port_arg_present) {
        println!("Required arguments were not provided, please follow the given usage: \n
        USAGE: cargo run -- --symbol <STOCKSYMBOL> --port <PORTNUMBER>\n
        or use '--help' for extra info");
        println!("Please Enter '<STOCKSYMBOL> <PORTNUMBER>' or 'exit' to end program: ");
        
        // Loop through until correct input obtained or user exits
        let mut input_symbol = String::new();
        loop {
            std::io::stdin().read_line(&mut input_symbol).expect("Failed to read line");
            let trimmed: Vec<&str> = input_symbol.split_ascii_whitespace().collect();   // Convert  whole symbol to uppercase

            if trimmed.len() == 2 {
                let symbol = trimmed[0];
                let port = trimmed[1];
                if is_valid_symbol(symbol) && is_valid_port(port){
                    return Args::parse_from(vec!["stockr", "--symbol", &trimmed[0].to_uppercase(), "--port", &trimmed[1]]);
                }
            }

            if trimmed[0].eq_ignore_ascii_case("exit") {
                println!("Exiting program.");
                std::process::exit(0);
            }

            println!("Invalid input. Please enter a value for each '<STOCKSYMBOL> <PORTNUMBER>' or type 'exit' to quit");
            input_symbol.clear();

        }
    }     
    
    // Convert  whole symbol to uppercase
    let mut args: Args = Args::parse();

    // Check if port is None and assign default value
    if args.port.is_none() {
        args.port = Some(8080);
    }
    
    // Extra Validation but should not trigger because of CLAP
    if is_valid_symbol(&args.symbol) && is_valid_port(&args.port.unwrap().to_string()) {
        args.symbol = args.symbol.to_uppercase();
        args
    } else {
        println!("Invalid symbol or port number. Please retry running the program. Exiting now!");
        std::process::exit(0);
    }
}
