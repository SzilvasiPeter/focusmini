#![forbid(unsafe_code)]
#[cfg(test)]
mod tests;

use std::env::split_paths;
use std::ffi::OsStr;
use std::io::{self, BufRead, Error, ErrorKind, Write, stdout};
use std::thread::sleep;
use std::time::Duration;

#[cfg(feature = "fast-tick")]
const TICK_DURATION: Duration = Duration::ZERO;

#[cfg(not(feature = "fast-tick"))]
const TICK_DURATION: Duration = Duration::from_secs(1);

pub trait Notifier {
    /// Alarm action that runs when a countdown reaches zero.
    ///
    /// # Errors
    /// Delegates to the implementation and yields any I/O error it produces.
    fn run(&self) -> io::Result<()>;
}

/// Parse optional work/break flags and return their minute values.
///
/// # Errors
/// Returns `Err` if a flag is unsupported, missing a value, or fails parsing/limits.
pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<(u16, u16), String> {
    let mut work = 60;
    let mut rest = 10;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "-w" | "--work" => work = parse_value(&flag, &mut args)?,
            "-b" | "--break" => rest = parse_value(&flag, &mut args)?,
            _ => return Err(format!("unknown flag {flag}")),
        }
    }

    Ok((work, rest))
}

/// Convert a flag string into minutes, capping at the configured maximum.
///
/// # Errors
/// Signals when a flag is missing a value, parsing fails, or the minutes exceed the limit.
pub fn parse_value(flag: &str, args: &mut impl Iterator<Item = String>) -> Result<u16, String> {
    const MAX: u16 = 1_080;
    let value = args.next().ok_or_else(|| format!("no value for {flag}"))?;
    let value = value
        .parse::<u16>()
        .map_err(|_| format!("invalid value '{value}' for {flag}"))?;
    (value <= MAX)
        .then_some(value)
        .ok_or_else(|| format!("{flag} value cannot exceed {MAX} minutes"))
}

/// Cycle between work and break timers, triggering `alarm` after each session.
///
/// # Errors
/// Any I/O failure from `countdown`, the notifier, or stdin/stdout is returned.
pub fn run(work: u16, brk: u16, alarm: &dyn Notifier, input: &mut dyn BufRead) -> io::Result<()> {
    let work_secs = work * 60;
    let break_secs = brk * 60;
    let work_session = ("\x1b[32m [Work] \x1b[0m", work_secs);
    let break_session = ("\x1b[34m [Break] \x1b[0m", break_secs);

    let mut input_line = String::new();
    for (label, seconds) in [work_session, break_session].into_iter().cycle() {
        countdown(label, seconds)?;
        alarm.run()?;

        print_flush("\r \x1b[1m Enter\x1b[0m to continue (\x1b[1mq\x1b[0m to quit): ")?;
        input_line.clear();

        let bytes = input.read_line(&mut input_line)?;
        if bytes >= 1 && input_line.trim_end_matches(['\r', '\n']) == "q" {
            break;
        }

        print_flush("\x1B[1A\x1B[2K")?; // move up and clear line
    }

    Ok(())
}

/// Print a countdown from `seconds` for the supplied label.
///
/// # Errors
/// Fails if writing to stdout or flushing fails.
pub fn countdown(label: &str, seconds: u16) -> io::Result<()> {
    for sec in (1..=seconds).rev() {
        print_flush(&format!("\r{label} {:02}:{:02}", sec / 60, sec % 60))?;
        sleep(TICK_DURATION);
    }

    print_flush(&format!("\r{label} 00:00"))?;
    Ok(())
}

/// Write `text` to stdout and flush immediately.
///
/// # Errors
/// Returns the flush error if the buffer cannot be written through.
pub fn print_flush(text: &str) -> io::Result<()> {
    print!("{text}");
    stdout().flush()?;
    Ok(())
}

/// Check `PATH` for a known audio player and return its executable name.
///
/// # Errors
/// Yields `ErrorKind::NotFound` when neither supported player is accessible.
pub fn find_audio_player(paths: &OsStr) -> io::Result<&'static str> {
    ["pw-play", "paplay"]
        .iter()
        .copied()
        .find(|name| split_paths(paths).any(|dir| dir.join(name).is_file()))
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "no audio player found"))
}
