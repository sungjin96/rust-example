const js = import("./node_modules/@sungjin5891/hello-wasm/hello_wasm.js");
js.then(js => {
    js.greet("WebAssembly");
});