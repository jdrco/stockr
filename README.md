## stockr

ECE 421 Project 1: Stock Market Monitor

# Crates Used:

chrono:
clap:
tokio:
yahoo_finance_api: This crate was used to gather the data of a specific stock symbol
plotters: This crate was used to plot the data points we got from the yahoo api to show a graphical representation

# Financial Analysis Algorithm:

# Charting Setup:

# Project Setup:

We made sure to modularize our code as much as possible. We split off each of the different functionalities in seperate files.

- main.rs: Calls the different parts of the application.
- analysis.rs: Contains all the functinality regarding the creation of the individual and daily plot points from the data we receive from yahoo api.
- cli.rs: Contains the functionality for command line parsing.
- plot.rs: Contains the logic that creates the graph with the plot points from analysis.rs.
- utils.rs: Contains helper functions, such as timestamp_to_local_date, determine_volatility and update_min_max_prices.

# Instructions:

1. Download the zip file or use git clone https://github.com/jdrco/stockr on your terminal
2. Run cargo build to install all the dependencies
3. Use the command cargo run -- --symbol s to generate the plot for a stock symbol, s represents the stock symbol
4. Check the directory plots-output, to find the image of the graph.
