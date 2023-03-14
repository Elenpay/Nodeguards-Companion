import { import_wasm } from './import_wasm';

(async () => {
  const module = await import_wasm();
  module.greet("Rodrigo");
})();