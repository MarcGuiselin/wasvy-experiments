# Start the app
start:
    just build-mods
    cargo run --release --features bevy/file_watcher

# Run
develop:
    bacon

# Run hot reloading for wasm
develop-mods:
    bacon hot-reload-mods

build-mods:
	cargo build --target wasm32-wasip2 --release --workspace --exclude wasvy-experiments --exclude shared
	cp target/wasm32-wasip2/release/*.wasm assets/mods/
