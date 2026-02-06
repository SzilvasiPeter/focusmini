.PHONY: build run lint

BINARY := target/release/focusmini

build:
	cargo build --release
	strip $(BINARY)

run: build
	$(BINARY)

lint:
	cargo clippy --all-targets -- -W clippy::all
	cargo audit
	cargo deny check
	cargo +nightly udeps
