# ci
lint:
	cargo clippy -- -D warnings
fmt:
	cargo fmt --all
test:
	cargo test --workspace --verbose
check: fmt lint test

# run examples
pong:
	cargo run --example=pong
sprite:
	cargo run --example=sprite
