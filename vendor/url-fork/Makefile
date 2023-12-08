precommit:
	#cargo fmt --all -- --config format_code_in_doc_comments=true
	cargo clippy
	cargo clippy --no-default-features
	cargo clippy --no-default-features --features serde
	cargo test
	cargo test --no-default-features
	cargo test --no-default-features --features serde

clean:
	cargo clean