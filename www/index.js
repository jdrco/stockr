class Chart {}

export function setup(WasmChart) {
  Chart = WasmChart;
}

async function runFetch(symbol) {
  try {
    return Chart.fetch_stock_data(symbol);
  } catch (error) {
    console.error('Error analyzing stock:', error);
  }
}

export async function main() {
  await runFetch('AAPL');
}