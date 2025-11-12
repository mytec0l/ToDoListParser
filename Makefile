fmt:
	cargo fmt --all

clippy:
	cargo clippy -- -D warnings

test:
	cargo test

check: 
	fmt clippy test

help:
	cargo run -- help

run:
	cargo run -- $(ARGS)

credits:
	cargo run -- credits