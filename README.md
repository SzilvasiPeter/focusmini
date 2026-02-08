# focusmini

Focusmini is a compact Pomodoro CLI that alternates between work and break intervals while triggering an audio cue on each transition.

> [!WARNING]
> Only works on Linux since it calls `pw-play` or `paplay` to play the Freedesktop alarm sound.

- ~100 lines of Rust
- small < 500 KiB release binary
- no external dependencies (std-only)
- zero unsafe code, 100% `lib.rs` test coverage

## Install

Install the prebuilt binary with `cargo-binstall` command:

```
cargo binstall focusmini
```

Or build from the source:

```
cargo install focusmini
```

## Usage

Run the `focusmini` with the deafult values or change them with options:

```
Usage: focusmini [OPTIONS]

Options:
  -w, --work <work_minutes>     Work interval length in minutes [default: 60]
  -b, --break <break_minutes>   Break interval length in minutes [default: 10]
```

## Commands

The table summary of the make commands:

| Make Target | Description |
|-------------|-------------|
| `make build` | Build the release binary and strip it. |
| `make run` | Build and execute the release binary. |
| `make lint` | Run Clippy in release mode against all targets with `clippy::all`. |
| `make test` | Run tests with the `fast-tick` feature in release mode. |
| `make cov` | Run test coverage with `cargo tarpaulin` command. |
| `make sec` | Run `cargo audit`, `cargo deny check`, and `cargo +nightly udeps --release`. |
