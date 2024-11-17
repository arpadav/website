# WASM instructions (for future reference)

1. Create new rust-wasm project

    ```bash
    cargo new <NAME> --lib
    ```

2. add the following to `Cargo.toml` (or respective versions of the libs):

    ```toml
    # ...

    [lib]
    crate-type = ["cdylib"]

    [dependencies]
    wasm-bindgen = "0.2"

    # helpful (but not necessary) dependencies
    js-sys = "0.3.65" # JS interface
    reqwasm = "0.2" # requests: paired with futures
    wasm-bindgen-futures = "0.4.38" # futures / async / await
    web-sys = { version = "0.3", features = ['console'] } # console logging

    [dev-dependencies]
    wasm-bindgen-test = "0.2"
    ```

3. build

    ```bash
    cargo build
    ```

4. build wasm

    ```bash
    wasm-pack build --release --target web --no-typescript --out-dir <OUT_DIR>
    ```

5. In your `.js` file, this is how to initialize and use:

    ```javascript
    import init, { <RUST_WASM_FUNCTIONS_AND_STRUCTS> } from './pkg/<PROJECT>.js';
    // this path is relative to the .js file                 ^^^^^^^^^^^^^^^^^^^

    async function init() {
        await init('./pkg/<PROJECT>_bg.wasm');
    }

    function example() {
        // if <RUST_WASM_FUNCTIONS_AND_STRUCTS> contains a normal element
        // let's say, a function called `rust_print` and it expects a &str
        rust_print("Hello!");
    }

    async function async_example() {
        // if <RUST_WASM_FUNCTIONS_AND_STRUCTS> contains an async element (with futures)
        // let's say, a function called `rust_request`
        await rust_request();
    }

    init();
    example();
    await async_example();
    ```
