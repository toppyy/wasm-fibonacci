# A Wasm-experiment

An application that converts an integer to it's representation in the fibonacci number system. A hello-world -type of thing for Rust + [Wasm](https://webassembly.org/).

Assuming proper tooling for Rust:
    
    git clone https://github.com/toppyy/wasm-fibonacci
    cd wasm-fibonacci
    wasm-pack build --target web
    python3 -m http.server

And go to: http://0.0.0.0:8000/


Material:
- [https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)