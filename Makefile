.PHONY: build run lint test cov sec

BINARY := target/release/focusmini

build:
	cargo build --release
	strip $(BINARY)

run: build
	$(BINARY)

lint:
	cargo clippy --all-targets --all-features --release

test:
	cargo test --release --all-targets --features fast-tick

cov:
	cargo tarpaulin --all-targets --features fast-tick --engine llvm

sec:
	cargo audit
	cargo deny check
	cargo +nightly udeps --release --all-targets
	cargo geiger
