.PHONY: build run lint security

BINARY := target/release/focusmini

build:
	cargo build --release
	strip $(BINARY)

run: build
	$(BINARY)

lint:
	cargo clippy --release --all-targets -- -W clippy::all

sec:
	cargo audit
	cargo deny check
	cargo geiger
	cargo +nightly udeps --release

test:
	cargo test --release --features fast-tick

cov:
	cargo tarpaulin --features fast-tick
