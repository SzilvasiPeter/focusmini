.PHONY: all build run lint test cov sec

BINARY := target/release/focusmini

all: lint test cov sec

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
	cargo tarpaulin --all-targets --features fast-tick --engine llvm --include-files 'src/main.rs' 'src/cli/mod.rs'

sec:
	cargo audit
	cargo deny check
	cargo +nightly udeps --release --all-targets
	cargo geiger
