##
##make cargo-*
cargo-help:### 	cargo-help
	@awk 'BEGIN {FS = ":.*?###"} /^[a-zA-Z_-]+:.*?###/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
cargo-build:### 	cargo build
## 	make cargo-build q=true
	@. $(HOME)/.cargo/env
	@RUST_BACKTRACE=all cargo b $(QUIET) --no-default-features
cargo-install:### 	cargo install --path .
#@. $(HOME)/.cargo/env
	#@cargo install --path $(PWD)
	@cargo install --locked --path $(PWD)
cargo-br:cargo-build-release### 	cargo-br
## 	make cargo-br q=true
cargo-build-release:### 	cargo-build-release
## 	make cargo-build-release q=true
	@. $(HOME)/.cargo/env
	@cargo b $(QUIET) --profile=$(PROFILE) --no-default-features
cargo-check:### 	cargo-check
## cargo c
	@. $(HOME)/.cargo/env
	@cargo c
	@cargo check --no-default-features --features tokio
	@cargo check --no-default-features --features async-std
cargo-bench:### 	cargo-bench
## cargo b
##	cargo build --release
	@. $(HOME)/.cargo/env
	@cargo bench
cargo-t:cargo-test### 	cargo-t
cargo-test:### 	cargo-test
## cargo t
	@. $(HOME)/.cargo/env && cargo test #--no-default-features
cargo-jits:### 	cargo-jits
## cargo-jits
	@. $(HOME)/.cargo/env && $(MAKE) cargo-t-jit-tokio cargo-t-jit-as
cargo-t-jit-tokio:cargo-test-js-interop-tokio### 	cargo-test-jit-tokio
cargo-test-js-interop-tokio:
	@. $(HOME)/.cargo/env && cargo test --no-default-features --features js_interop_tests,tokio
cargo-t-jit-as:cargo-test-js-interop-async-std### 	cargo-test-jit-as
cargo-test-js-interop-async-std:
	@. $(HOME)/.cargo/env && cargo test --no-default-features --features js_interop_tests,async-std
cargo-test-benches:### 	cargo-test
	cargo test --benches
cargo-report:### 	cargo-report
	@. $(HOME)/.cargo/env && cargo report future-incompatibilities --id 1
cargo-doc:### 	cargo-doc
	@. $(HOME)/.cargo/env && cargo doc --no-deps #--open
cargo-b-wasm-tokio:
	@. $(HOME)/.cargo/env && cargo clean && cargo build --target=wasm32-unknown-emscripten --no-default-features #--features wasm-bindgen,tokio

# vim: set noexpandtab:
# vim: set setfiletype make
