ALL:
	pushd client && cargo build && popd && echo "done"
	wasm-tools component new --skip-validation   ./target/wasm32-wasip1/debug/guest.wasm  --adapt ./wasi_snapshot_preview1.reactor.wasm -o wasm_modules/guest.wasm
	pushd server && cargo run
