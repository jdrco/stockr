<p align="center">
    <img width="700" alt="image" src="https://github.com/jdrco/stockr/assets/83478026/80f48c7d-1a44-419c-838a-52222d30ef3b">
</p>

# stockr

A Stock Market Monitor.

Contributors:
- Jared Drueco
- Antonio Martin-O
- Pratham Sitoula
- Muhammed Rashid

# Crates Used:

`chrono`: This crate is used to handle the date and time of the stock quotes.

`clap`: This crate is used to parse the command line arguments.

`tokio`: This crate is used as an asynchronous runtime to handle the requests to the yahoo_finance_api.

`yahoo_finance_api`: This crate was used to gather the data of a specific stock symbol.

`plotters`: This crate was used to generate a graph from the processed data points we got from analysis.rs.

# Financial Analysis Algorithm:

We extracted the main analysis into a separate module called `analysis.rs` which contains a custom struct `StockAnalysis` that holds the relevant stock data and the methods to calculate the different metrics.

### Min/Max Closing Price Calculations:

While iterating over each daily quote, we keep track of the minimum and maximum closing price. If the current quote's closing price is lower than the current minimum, we update the minimum value, and if it's higher than the current maximum, we update the maximum value.


### Volatility Calculations:

When iterating over each daily quote, we calculate the volatility of each one by passing in its daily low and daily high to check if there's a difference of at least 2%, the stock is regarded as volatile if the condition is met:

```rust
pub fn determine_volatility(high: f64, low: f64) -> bool {
    (high - low) / low > 0.02
}
```

This helps us separate quotes into two categories: volatile and regular quotes. We use this information to highlight volatile and regular data points in the graph:
```rust
pub regular_quotes: Vec<DailyQuote>,
pub volatile_quotes: Vec<DailyQuote>,
```

# Charting Setup:

We used the `plotters` crate to generate the graph. We used the `BitMapBackend` to create the graph and the `ChartBuilder` to add the different elements to the graph.

### Visual Design Choice:

We chose to use a candle stick chart to represent the stock data. We decided to use this over a scatter plot as it makes it easier to compare daily quotes. We used red and green colours to represent whether the daily quote closed lower or higher than the opening price.

To highlight volatile data points, we use a solid colour to fill in the data point, whereas an empty bordered box represents regular data points. This was done by drawing a series for quote vectors `regular_quotes` and `volatile_quotes`. This is described in the legend we provided for the user.

### Autoscaling / Axis Features:

To ensure that our graph is readable and shows all data points within the current range, we set the range of the x and y-axis. To achieve `autoscaling`, we updated the absolute min and max values (the lowest and highest prices within the range respectively) and used those as the vertical constraints for the y-axis. Similarly for the x-axis, we used the date of the first and last data point (with an offset for side margins) to set the horizontal constraints.

# Project Setup:

We made sure to *modularize* our code as much as possible. We split off each of the different functionalities into separate modules.

- `main.rs` Contains the main entry point of the application.
- `analysis.rs`: Contains all the functionality regarding the creation of the daily plot points and determining its volatility from the data we receive from yahoo api. Serves as the main module for the financial analysis.
- `cli.rs`: Contains the functionality for command line parsing.
- `plot.rs`: Contains the logic that creates the graph with the plot points from analysis.rs.
- `utils.rs`: Contains helper functions.
- `lib.rs`: Contains the imports that help centralize the connection of all modules.

# Usage Instructions:

1. Download the zip file or use `git clone https://github.com/jdrco/stockr` on your terminal and `cd` into that directory.
2. Run `cargo build` to install all the dependencies.
3. Use the command `cargo run -- --symbol <SYMBOL>` to generate the plot for a stock symbol.
4. You can use `cargo run -- --help` for a user input guide.
5. Check the directory `plots-output`, to find the image of the graph.
