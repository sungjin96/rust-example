const js = import("@sungjin5891/hello-wasm/hello_wasm.js");
js
    .then(js => js.greet("WebAssembly"))
    .catch(console.error)
