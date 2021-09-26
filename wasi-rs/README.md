In order to build it, we first need to install a WASI-enabled Rust toolchain:

> rustup target add wasm32-wasi
 
> cargo build --target wasm32-wasi

> wasmer --dir=. --dir=/tmp ./target/wasm32-wasi/debug/wasi-rs.wasm test.txt /tmp/somewhere.txt

demo.wat

To run this example within the browser, simply upload the compiled `.wasm` file to the [WASI browser polyfill](https://wasi.dev/polyfill/)

