build:
	cd roguelike && cargo +nightly build --release --target wasm32-unknown-unknown
	cp roguelike/target/wasm32-unknown-unknown/release/roguelike.wasm .
