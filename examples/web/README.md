# A Web Example

## Install `wasm-pack` and `http`
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install https
```

## Build & Run Web Server
```bash
wasm-pack build --target web && http .
```

Now, open http://localhost:8000/ in your web browser and check the javascript
console.
