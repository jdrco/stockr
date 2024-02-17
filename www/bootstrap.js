init();

async function init() {
  const [{ Chart, default: init }, { main, setup }] = await Promise.all([
    import('../pkg/stockr.js'),
    import('./index.js'),
  ]);
  await init();
  setup(Chart);
  await main();
}
