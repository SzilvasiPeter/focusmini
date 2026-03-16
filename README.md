# focusmini

Focusmini is a compact Pomodoro CLI that alternates between work and break intervals while playing a short beep pattern on each transition.

- compact codebase
- minimal dependencies (uses `rodio` for audio cues)
- zero unsafe code, test coverage: <https://szilvasipeter.github.io/focusmini/coverage/index.html>

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

Run the `focusmini` with the default values or change them with options:

```
Usage: focusmini [OPTIONS]

Options:
  -w, --work <work_minutes>     Work interval length in minutes [default: 60]
  -b, --break <break_minutes>   Break interval length in minutes [default: 10]
```

## Commands

The table summary of the `just` commands:

| Just Command | Description |
|--------------|-------------|
| `just build` | Build the release binary and strip it. |
| `just run` | Build and execute the release binary. |
| `just lint` | Run Clippy in release mode against all targets with `clippy::all`. |
| `just test` | Run tests with the `fast-tick` feature in release mode. |
| `just cov` | Run test coverage with `cargo tarpaulin` command. |
| `just sec` | Run `cargo audit`, `cargo deny check`, `cargo geiger` and `cargo +nightly udeps --release`. |
