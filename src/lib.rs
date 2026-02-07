#![forbid(unsafe_code)]
use std::io::{self, BufRead, Write, stdout};

use std::time::Duration;

#[cfg(feature = "fast-tick")]
const SECONDS_PER_MINUTE: u16 = 1;

#[cfg(not(feature = "fast-tick"))]
const SECONDS_PER_MINUTE: u16 = 60;

#[cfg(feature = "fast-tick")]
const TICK_DURATION: Duration = Duration::from_millis(1);

#[cfg(not(feature = "fast-tick"))]
const TICK_DURATION: Duration = Duration::from_secs(1);

pub trait Notifier {
    fn run(&self) -> io::Result<()>;
}

pub fn parse_args<I>(mut args: I) -> Result<(u16, u16), String>
where
    I: Iterator<Item = String>,
{
    args.next();
    let mut work = 60;
    let mut rest = 10;

    while let Some(flag) = args.next() {
        let error = format!("missing value for {}", flag);
        let value = args.next().ok_or(error)?;
        match flag.as_str() {
            "-w" | "--work" => work = parse_value(flag.as_str(), value.as_str())?,
            "-b" | "--break" => rest = parse_value(flag.as_str(), value.as_str())?,
            _ => drop(flag),
        }
    }

    Ok((work, rest))
}

pub fn parse_value(flag: &str, value: &str) -> Result<u16, String> {
    let error = format!("invalid value for {}", flag);
    value.parse::<u16>().map_err(|_| error)
}

pub fn run(work: u16, brk: u16, alarm: &dyn Notifier, input: &mut dyn BufRead) -> io::Result<()> {
    let work_secs = work * SECONDS_PER_MINUTE;
    let break_secs = brk * SECONDS_PER_MINUTE;
    let work = ("\x1b[1m [Work] \x1b[0m", work_secs);
    let pause = ("\x1b[1m [Break] \x1b[0m", break_secs);

    let mut input_line = String::new();
    for (label, seconds) in [work, pause].into_iter().cycle() {
        countdown(label, seconds)?;
        alarm.run()?;

        print_flush("\rEnter to continue (q to quit): ")?;
        input_line.clear();
        input.read_line(&mut input_line)?;
        if matches!(input_line.chars().next(), Some('q')) {
            break;
        }

        print_flush("\x1B[1A\x1B[2K")?; // move up and clear line
    }

    Ok(())
}

pub fn countdown(label: &str, seconds: u16) -> io::Result<()> {
    for sec in (1..=seconds).rev() {
        print_flush(&format!("\r{label} {:02}:{:02}", sec / 60, sec % 60))?;
        std::thread::sleep(TICK_DURATION);
    }

    print_flush(&format!("\r{label} 00:00"))?;
    Ok(())
}

pub fn print_flush(text: &str) -> io::Result<()> {
    print!("{text}");
    stdout().flush()?;
    Ok(())
}
