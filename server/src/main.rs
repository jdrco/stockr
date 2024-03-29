mod analysis;
mod cli;
use crate::analysis::{determine_volatility, timestamp_to_local_date, update_min_max_prices, AppState, StockAnalysis, DailyQuote};
use actix_web::{get, web, Responder, Result, HttpResponse, App, HttpServer};
use actix_files::NamedFile;
use cli::parse_args;
use chrono::NaiveDate;
use yahoo_finance_api as yahoo;
use actix_files as fs;
use std::sync::Mutex;

// Function for analyzing and plotting stock data
#[get("/stock/{symbol}")]
async fn analyze_stock(data: web::Data<Mutex<AppState>>, symbol: web::Path<String>) -> Result<impl Responder> {
    // Lock mutex and change data
    let mut app_state = data.lock().unwrap();
    app_state.user_input = Some(symbol.to_string());

    // API call to get stock data from Yahoo Stocks
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_range(&symbol, "1d", "6mo").await;

    // Formatting stock data when stocks are correctly fetched
    match response {
        Ok(response) => {
            let quotes = response.quotes();
            match quotes {
                Ok(quotes) => {
                    let mut analysis = StockAnalysis {
                        min_close_price: std::f64::MAX,
                        max_close_price: std::f64::MIN,
                        min_close_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
                        max_close_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
                        start_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
                        end_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
                        min_low_price: std::f64::MAX,
                        max_high_price: std::f64::MIN,
                        regular_quotes: Vec::new(),
                        volatile_quotes: Vec::new(),
                    };
        
                    if let Some(first_quote) = quotes.first() {
                        analysis.start_date =
                            timestamp_to_local_date(first_quote.timestamp.try_into().unwrap());
                    }
                    if let Some(last_quote) = quotes.last() {
                        analysis.end_date = timestamp_to_local_date(last_quote.timestamp.try_into().unwrap());
                    }
            
                    for quote in quotes {
                        let local_date = timestamp_to_local_date(quote.timestamp.try_into().unwrap());
            
                        let volatility = determine_volatility(quote.high, quote.low);
                        let daily_quote = DailyQuote {
                            date: local_date,
                            open: quote.open,
                            high: quote.high,
                            low: quote.low,
                            volume: quote.volume,
                            close: quote.close,
                            adjclose: quote.adjclose,
                            is_volatile: volatility,
                        };
            
                        if volatility {
                            analysis.volatile_quotes.push(daily_quote);
                        } else {
                            analysis.regular_quotes.push(daily_quote);
                        }
            
                        update_min_max_prices(&mut analysis, quote, local_date);
                    }

                    println!("Analysis Report: {}", app_state.user_input.as_ref().unwrap());
                    println!(
                        "Min Price: {:.2} on {}",
                        analysis.min_close_price, analysis.min_close_date
                    );
                    println!(
                        "Max Price: {:.2} on {}",
                        analysis.max_close_price, analysis.max_close_date
                    );
                    
                
                    let json_string = 
                    serde_json::to_string(&analysis).unwrap();
                    Ok(HttpResponse::Ok().body(json_string))
                },
                Err(err) => {
                    Ok(HttpResponse::InternalServerError().body(format!("Failed to get quotes: {}", err))) // Error handling
                }
            }
        
        },
        Err(err) => {
            Ok(HttpResponse::InternalServerError().body(format!("Failed to get quotes: {}", err)))  // Error Handling
        }
    }
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    // Specify the path to your index.html file
    Ok(NamedFile::open("../www/index.html")?)
}

#[get("/stock")]
async fn get_user_input(data: web::Data<Mutex<AppState>>) -> HttpResponse {
    // Access the application state using the `Data` extractor
    let app_state = data.lock().unwrap();
    if let Some(user_input) = &app_state.user_input {
        HttpResponse::Ok().body(format!("{}", user_input))
    } else {
        HttpResponse::BadRequest().body("No user input stored")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // Create a mutable string to store the user input
    let input = parse_args();
    
    // Create application state with user input
    let app_state = 
        web::Data::new(Mutex::new(AppState {user_input: Some(input.symbol.to_string())}));
    
    let address = format!("127.0.0.1:{}", input.port.unwrap());
    println!("Server running on http://{}/", &address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(analyze_stock)
            .service(get_user_input)
            .service(fs::Files::new("/pkg", "../pkg").show_files_listing()) // Serve the WASM package
            .service(fs::Files::new("/", "../www").index_file("index.html")) // Serve your static files
    })
    .bind(address)?
    .run()
    .await
}