# What is this?

This is a simple experiment to show how to make a simple roguelike interface using Rust and Web Assembly. There's not much to it.

https://richardanaya.github.io/rust-roguelike/index.html

Interesting detail: Rust doesn't allow mutable static globals. So you have to create a global mutex that holds a mutable value. That way all your library entrypoint functions sharing that global data are thread safe. Turns out there is a common package lazy_static that makes this easy.

Feel free to leave an issue if you see a more idiomatic way of doing something.

# How to build

```bash
curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain=nightly
rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib roguelike.rs -o roguelike.wasm
```
