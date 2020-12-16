### Build with wasm-pack
Use: https://github.com/rustwasm/wasm-pack
```
wasm-pack build
```

### Build Manually
```
cargo build --target wasm32-unknown-unknown

# browser
wasm-bindgen ./target/wasm32-unknown-unknown/debug/ed25519.wasm --out-dir pkg --target web

# nodejs
wasm-bindgen ./target/wasm32-unknown-unknown/debug/ed25519.wasm --out-dir pkg --target nodejs
```
