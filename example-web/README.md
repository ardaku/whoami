# A Web Example

## Install `wasm-pack` and `http`
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install simple-http-server --locked
```

## Build & Run Web Server
```bash
wasm-pack build --target web && simple-http-server .
```

Now, open http://localhost:8000/index.html in your web browser and check the
javascript console.
