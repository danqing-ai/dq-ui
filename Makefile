.PHONY: studio-create studio-build fmt clippy test

studio-create:
	cargo run -p studio-create

studio-build:
	cargo build -p studio-create

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings

test:
	cargo test --workspace
