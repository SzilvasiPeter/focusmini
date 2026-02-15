binary := "target/release/focusmini"

all: lint test cov sec

build:
    cargo build --release
    strip {{binary}}

run: build
    {{binary}}

lint:
    cargo clippy --all-targets --all-features --release

test:
    cargo test --release --all-targets --features fast-tick

cov:
    cargo tarpaulin --all-targets --features fast-tick --engine llvm \
      --include-files 'src/main.rs' 'src/cli/mod.rs'

sec:
    cargo audit
    cargo deny check
    cargo +nightly udeps --release --all-targets
    cargo geiger
