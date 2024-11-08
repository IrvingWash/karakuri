# ci
lint:
	cargo clippy --workspace -- -D warnings
fmt:
	cargo fmt --all
test:
	cargo test --workspace
check: fmt lint test

# run examples
pong:
	cargo run --example=pong
collisions:
	cargo run --example=collisions
shmup:
	cargo run --example=shmup
racing:
	cargo run --example=racing
