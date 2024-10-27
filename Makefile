# ci
lint:
	cargo clippy -- -D warnings
fmt:
	cargo fmt --all
test:
	cargo test --workspace --verbose
check: fmt lint test

# run examples
canary:
	cargo run --example=canary
pong:
	cargo run --example=pong
