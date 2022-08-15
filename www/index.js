async function init() {
    const res = await fetch("sum.wasm");
    buffer = await res.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer);

    const sumFunction = wasm.instance.exports.sum;
    const result = sumFunction(70, 3000);
    console.log(result);
  }
  
  init();
