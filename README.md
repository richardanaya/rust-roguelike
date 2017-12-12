```bash
curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain=nightly
make
```
