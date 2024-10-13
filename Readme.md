# it works with _trunk_
`trunk serve --open`


# it also works with `basic-http-server`
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

