class Chart {}

// Sets up the Chart with the WasmChart module.
export function setup(WasmChart) {
  Chart = WasmChart;
}

// Fetches and displays stock data, handling errors by showing an error message.
async function runFetchStockData(symbol) {
  try {
    const result = await Chart.fetch_stock_data(symbol);
    document.getElementById('errorText').style.display = 'none';
    return result;
  } catch (error) {
    console.error('Error analyzing stock:', error);
    document.getElementById('errorText').style.display = 'inline';
    return null;
  }
}

// Initializes the form with a default symbol and fetches its data.
async function initializeSymbolInput() {
  let symbol = null;
  try {
    symbol = await Chart.fetch_symbol();
  } catch (error) {
    console.error('Error fetching initial symbol:', error);
  }
  if (symbol) {
    document.getElementById('symbolInput').value = symbol;
    await runFetchStockData(symbol);
  }
}

// Main setup for the form submission and initial page load actions.
export async function main() {
  document.getElementById('stockForm').addEventListener('submit', async function (event) {
    event.preventDefault();
    const symbol = document.getElementById('symbolInput').value;
    await runFetchStockData(symbol);
  });

  await initializeSymbolInput();
}
