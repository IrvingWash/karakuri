# ci
lint:
	cargo clippy --workspace -- -D warnings
fmt:
	cargo fmt --all
test:
	cargo test --workspace
check: fmt lint test
coverage:
	cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out html

# chores
clean:
	rm -rf ./target

# run examples
pong:
	cargo run --example=pong
collisions:
	cargo run --example=collisions
shmup:
	cargo run --example=shmup
racing:
	cargo run --example=racing
canary:
	cargo run --package=kphysics --example=canary
