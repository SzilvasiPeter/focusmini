.PHONY: build run lint security

BINARY := target/release/focusmini

build:
	cargo build --release
	strip $(BINARY)

run: build
	$(BINARY)

lint: build
	cargo clippy --release --all-targets -- -W clippy::all

sec: build
	cargo audit
	cargo deny check
	cargo +nightly udeps --release

test:
	cargo test --release
