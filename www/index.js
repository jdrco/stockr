class Chart {}

export function setup(WasmChart) {
  Chart = WasmChart;
}

async function runFetchStockData(symbol) {
  try {
    return Chart.fetch_stock_data(symbol);
  } catch (error) {
    console.error('Error analyzing stock:', error);
  }
}

async function initializeSymbolInput() {
  try {
    const symbol = await Chart.fetch_symbol(); 
    console.log(symbol)
    if (symbol) {
      document.getElementById('symbolInput').value = symbol;
      await runFetchStockData(symbol);
    }
  } catch (error) {
    console.error('Error fetching initial symbol:', error);
  }
}

export async function main() {
  document.getElementById('stockForm').addEventListener('submit', async function(event) {
    event.preventDefault();
    const symbol = document.getElementById('symbolInput').value;
    await runFetchStockData(symbol);
  });

  await initializeSymbolInput();
}