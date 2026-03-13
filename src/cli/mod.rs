#![forbid(unsafe_code)]
#[cfg(test)]
mod tests;

use std::io::{self, BufRead, Write, stdout};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::DEFAULT_SOUND;

#[cfg(feature = "fast-tick")]
const TICK_DURATION: Duration = Duration::ZERO;

#[cfg(not(feature = "fast-tick"))]
const TICK_DURATION: Duration = Duration::from_secs(1);

pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<(u16, u16, String), String> {
    let mut work = 60;
    let mut rest = 10;
    let mut sound = DEFAULT_SOUND.to_string();

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "-w" | "--work" => work = parse_value(&flag, &mut args)?,
            "-b" | "--break" => rest = parse_value(&flag, &mut args)?,
            "-s" | "--sound" => {
                sound = args.next().ok_or_else(|| format!("no value for {flag}"))?;
            }
            _ => return Err(format!("unknown flag {flag}")),
        }
    }

    Ok((work, rest, sound))
}

pub fn run(work: u16, brk: u16, sound: &str, input: &mut dyn BufRead) -> io::Result<()> {
    let work_secs = work * 60;
    let break_secs = brk * 60;
    let work_session = ("\x1b[32m [Work] \x1b[0m", work_secs);
    let break_session = ("\x1b[34m [Break] \x1b[0m", break_secs);

    let mut input_line = String::new();
    for (label, seconds) in [work_session, break_session].into_iter().cycle() {
        countdown(label, seconds)?;
        play_sound(sound)?;

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

fn parse_value(flag: &str, args: &mut impl Iterator<Item = String>) -> Result<u16, String> {
    const MAX: u16 = 1_080;
    let value = args.next().ok_or_else(|| format!("no value for {flag}"))?;
    let value = value
        .parse::<u16>()
        .map_err(|_| format!("invalid value '{value}' for {flag}"))?;
    (1..=MAX)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{flag} value should be between 1 and {MAX} minutes"))
}

fn countdown(label: &str, seconds: u16) -> io::Result<()> {
    for sec in (1..=seconds).rev() {
        print_flush(&format!("\r{label} {:02}:{:02}", sec / 60, sec % 60))?;
        sleep(TICK_DURATION);
    }

    print_flush(&format!("\r{label} 00:00"))?;
    Ok(())
}

fn play_sound(sound: &str) -> io::Result<()> {
    if sound == "none" {
        return Ok(());
    }
    let _ = Command::new("paplay").arg(sound).status()?;
    Ok(())
}

fn print_flush(text: &str) -> io::Result<()> {
    print!("{text}");
    stdout().flush()?;
    Ok(())
}
