// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
// import("./index.js")
//   .catch(e => console.error("Error importing `index.js`:", e));

import('../pkg/stockr_wasm.js');
import('./index.js');

init();

async function init() {
  const [{ run, greet }, { main, setup }] = await Promise.all([
    import('../pkg/stockr_wasm.js'),
    import('./index.js'),
  ]);
  await init();
  setup(greet);
  main();
}
