# Start the app
start:
    just build-mods
    cargo run --release --features bevy/file_watcher

# Run hot reloading for wasm
develop:
    bacon -j hot-reload-mods

build-mods:
	cargo build --target wasm32-wasip2 --release --workspace --exclude wasvy-experiments --exclude shared
	cp target/wasm32-wasip2/release/*.wasm assets/mods/
