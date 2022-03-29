## Infinityswap Dfinity Executor

### Setup
- clone the identity repo
    ```
    git clone https://github.com/infinity-swap/identity
    ```
- clone the repo
- run below to compile this binary crate
    ```
    cargo build
    ```
- for a release build, run
    ```
    cargo build --release && cp target/release/ifx .
    ```

### Usage
- run below to view usage description for this cli
    ```
    ./ifx --help
    ```
- example cmd to upload wasm to token canister
    ```
    ./ifx wasm-upload --identity=<path to pem file> [--network=[local|ic|<url>]] wasm-upload --path=<path to wasm> <CANISTER_ID>
    ```