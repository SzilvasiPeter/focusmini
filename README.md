# focusmini

Focusmini is a compact Pomodoro CLI that alternates between work and break intervals while triggering an audio cue on each transition.

> [!NOTE]
> The CLI plays a sound with `paplay` on each transition. Use `--sound none` to disable it or set a custom sound with `--sound <path>`.

- ~100 LOC production code
- small < 500 KiB release binary
- no external dependencies (std-only)
- zero unsafe code, >90% test [coverage](https://szilvasipeter.github.io/focusmini/coverage/index.html)

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
  -s, --sound <path>            Sound file path or `none` to disable [default: /usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga]
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
