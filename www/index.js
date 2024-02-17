class Chart {}

export function setup(WasmChart) {
  Chart = WasmChart;
}

async function runFetchStockData(symbol) {
  try {
    const result = await Chart.fetch_stock_data(symbol);
    document.getElementById('errorText').style.display = 'none';
    return result;
  } catch (error) {
    console.error('Error analyzing stock:', error);
    document.getElementById('errorText').style.display = 'inline';
    document.getElementById('errorText').style.display = 'inline';
    return null;
  }
}

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

export async function main() {
  document
    .getElementById('stockForm')
    .addEventListener('submit', async function (event) {
      event.preventDefault();
      const symbol = document.getElementById('symbolInput').value;
      await runFetchStockData(symbol);
    });

  await initializeSymbolInput();
}
