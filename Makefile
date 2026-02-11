.PHONY: build run lint security

BINARY := target/release/focusmini

build:
	cargo build --release
	strip $(BINARY)

run: build
	$(BINARY)

lint:
	cargo clippy --release --all-targets -- -W clippy::all

test:
	cargo test --release --all-targets --features fast-tick

cov:
	cargo tarpaulin --all-targets --features fast-tick

sec:
	cargo audit
	cargo deny check
	cargo +nightly udeps --release --all-targets
	cargo geiger
