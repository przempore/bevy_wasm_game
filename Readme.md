# it works with _trunk_
`trunk serve --open`


# it also works with `basic-http-server`
## install basic-http-server
```shell
cargo install basic-http-server
```
## install wasm-bindgen
```shell
cargo install wasm-bindgen-cli
```
## Build
```shell
❯ cargo build --release --target wasm32-unknown-unknown
❯ wasm-bindgen --out-name bevy_game_menu \
              --out-dir ./target \
              --target web target/wasm32-unknown-unknown/release/bevy_game_menu.wasm
```
## Run
```shell
❯ basic-http-server .
```

