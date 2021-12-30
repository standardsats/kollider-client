The library contains REST and WebSocket bindings for API of [Kollider](https://kollider.xyz) trading platform.

The bindings follow official [API documentation](https://docs-api.kollider.xyz/) except some parts that are underdocumented or different at the moment.

The library contains also CLI interface for calling the endpoints and WS commands. To build it you have to run:
```
cargo run --release --features="build-binary"
```

More complex example that places an order via WebSocket:
```
RUST_LOG=debug cargo run --release --features="build-binary" -- websocket private index_values --symbols .BTCUSD order --price 472520 --quantity 1 --side ask
```