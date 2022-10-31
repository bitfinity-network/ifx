## Infinityswap Dfinity Executor

### Build
```
cargo build
```

For a release build, run

```
cargo build --release && cp target/release/ifx 
```

### Usage
Run below to view usage description for this cli

```sh
./ifx --help
```
Example cmd to upload wasm to token canister
```sh
./ifx wasm-upload --identity=<path to pem file> [--network=[local|ic|<url>]] wasm-upload --path=<path to wasm> <CANISTER_ID>
```

Note that **--identity** option should point to *.pem to the active idendity in DFX configuration
