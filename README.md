## stockr

ECE 421 Project 1: Stock Market Monitor

# Crates Used:

chrono:
clap:
tokio:
yahoo_finance_api: This crate was used to gather the data of a specific stock symbol.
plotters: This crate was used to generate a graph from the data points we got from the yahoo api.

# Financial Analysis Algorithm:

To determine whether a specific data point is volatile, we implemented a simple algorithm:

pub fn determine_volatility(high: f64, low: f64) -> bool {
(high - low) / low > 0.02
}

To determine whether a daily quote is volatile, we pass in its daily low and daily high. Then we check if there's a difference of at least 2%, the stock is regarded as volatile if the condition is met.

# Charting Setup:

# Project Setup:

We made sure to modularize our code as much as possible. We split off each of the different functionalities in seperate files.

- main.rs: Calls the different parts of the application.
- analysis.rs: Contains all the functionality regarding the creation of the daily plot points and determining its volatility from the data we receive from yahoo api.
- cli.rs: Contains the functionality for command line parsing.
- plot.rs: Contains the logic that creates the graph with the plot points from analysis.rs.
- utils.rs: Contains helper functions, such as timestamp_to_local_date, determine_volatility and update_min_max_prices.

# Instructions:

1. Download the zip file or use git clone https://github.com/jdrco/stockr on your terminal
2. Run cargo build to install all the dependencies
3. Use the command cargo run -- --symbol s to generate the plot for a stock symbol, s represents the stock symbol
4. Check the directory plots-output, to find the image of the graph.
