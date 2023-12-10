##
##make cargo-*
cargo-help:### 	cargo-help
	@awk 'BEGIN {FS = ":.*?###"} /^[a-zA-Z_-]+:.*?###/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

cargo-bt:cargo-build-tokio### 	cargo-bt
cargo-build-tokio:### 	cargo-build-tokio
## 	make cargo-build-tokio q=true
	@. $(HOME)/.cargo/env
	@RUST_BACKTRACE=all cargo b $(QUIET) --no-default-features --features tokio

cargo-bas:cargo-build-async-std### 	cargo-bas
cargo-build-async-std:### 	cargo-build-async-std
## 	make cargo-build-async-std q=true
	@. $(HOME)/.cargo/env
	@RUST_BACKTRACE=all cargo b $(QUIET) --no-default-features --features async-std

cargo-install:### 	cargo install --path .
#@. $(HOME)/.cargo/env
	#@cargo install --path $(PWD)
	@cargo install --locked --path $(PWD)

cargo-br-tokio:cargo-build-release-tokio### 	cargo-br-tokio
## 	make cargo-br-tokio q=true

cargo-build-release-tokio:### 	cargo-build-release-tokio
## 	make cargo-build-release-tokio q=true
	@. $(HOME)/.cargo/env
	@cargo b $(QUIET) --profile=$(PROFILE) --no-default-features --features tokio

cargo-br-async-std:cargo-build-release-async-std### 	cargo-br-async-std
## 	make cargo-br-async-std q=true

cargo-build-release-async-std:### 	cargo-build-release-async-std
## 	make cargo-build-release-async-std q=true
	@. $(HOME)/.cargo/env
	@cargo b $(QUIET) --profile=$(PROFILE) --no-default-features --features async-std

cargo-check:### 	cargo-check
## cargo c
	@. $(HOME)/.cargo/env
	@cargo c

cargo-check-tokio:### 	cargo-check-tokio
## cargo c --no-default-features --features tokio
	@. $(HOME)/.cargo/env
	@cargo check --no-default-features --features tokio

cargo-check-async-std:### 	cargo-check-async-std
## cargo c --no-default-features --features async-std
	@. $(HOME)/.cargo/env
	@cargo check --no-default-features --features async-std

cargo-bench:### 	cargo-bench
## cargo bench
	@. $(HOME)/.cargo/env
	@cargo bench

cargo-tt:cargo-test-tokio### 	cargo-tt
cargo-test-tokio:### 	cargo-test-tokio
## cargo tt
	@. $(HOME)/.cargo/env && cargo test --no-default-features --features tokio

cargo-ts:cargo-test-async-std### 	cargo-ts
cargo-test-async-std:### 	cargo-test-async-std
## cargo ts
	@. $(HOME)/.cargo/env && cargo test --no-default-features --features async-std

cargo-jits:### 	cargo-jits
## cargo-jits
	@. $(HOME)/.cargo/env && $(MAKE) cargo-t-jit-tokio cargo-t-jit-as

cargo-t-jit-tokio:cargo-test-js-interop-tokio### 	cargo-test-jit-tokio
cargo-test-js-interop-tokio:
	@. $(HOME)/.cargo/env && cargo test --no-default-features --features js_interop_tests,tokio

cargo-t-jit-as:cargo-test-js-interop-async-std### 	cargo-test-jit-as
cargo-test-js-interop-async-std:
	@. $(HOME)/.cargo/env && cargo test --no-default-features --features js_interop_tests,async-std

cargo-test-benches:### 	cargo-test-benches
	cargo test --benches

cargo-report:### 	cargo-report
	@. $(HOME)/.cargo/env && cargo report future-incompatibilities --id 1

cargo-doc:### 	cargo-doc
	@. $(HOME)/.cargo/env && cargo doc --no-deps #--open

cargo-b-wasm-tokio:
	@rustup target add wasm32-unknown-emscripten
	@. $(HOME)/.cargo/env && cargo clean && cargo build --target=wasm32-unknown-emscripten --no-default-features --features wasm-bindgen,tokio

cargo-b-wasm-async-std:
	@rustup target add wasm32-unknown-emscripten
	@. $(HOME)/.cargo/env && cargo clean && cargo build --target=wasm32-unknown-emscripten --no-default-features --features wasm-bindgen,async-std

node-sdk-run-js-node:### 	node-sdk-run-js-node
## 	node-sdk-run-js-node
	node sdk/run.js node
node-sdk-run-js-node-6102:### 	node-sdk-run-js-node-6102
## 	node-sdk-run-js-node-6102
	node sdk/run.js node 6102
node-sdk-run-js-rust:### 	node-sdk-run-js-rust
## 	node-sdk-run-js-rust
	node sdk/run.js rust
node-sdk-run-js-rust-2106:### 	node-sdk-run-js-rust-2106
## 	node-sdk-run-js-rust-2106
	node sdk/run.js rust 2106

# vim: set noexpandtab:
# vim: set setfiletype make
