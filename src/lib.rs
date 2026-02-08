#![forbid(unsafe_code)]
use std::io::{self, BufRead, Error, ErrorKind, Write, stdout};
use std::time::Duration;

#[cfg(feature = "fast-tick")]
const TICK_DURATION: Duration = Duration::ZERO;

#[cfg(not(feature = "fast-tick"))]
const TICK_DURATION: Duration = Duration::from_secs(1);

pub trait Notifier {
    fn run(&self) -> io::Result<()>;
}

pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<(u16, u16), String> {
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
    value
        .parse::<u16>()
        .map_err(|_| format!("invalid value for {}", flag))
}

pub fn run(work: u16, brk: u16, alarm: &dyn Notifier, input: &mut dyn BufRead) -> io::Result<()> {
    let work_secs = work * 60;
    let break_secs = brk * 60;
    let work = ("\x1b[32m [Work] \x1b[0m", work_secs);
    let pause = ("\x1b[34m [Break] \x1b[0m", break_secs);

    let mut input_line = String::new();
    for (label, seconds) in [work, pause].into_iter().cycle() {
        countdown(label, seconds)?;
        alarm.run()?;

        print_flush("\r \x1b[1m Enter\x1b[0m to continue (\x1b[1mq\x1b[0m to quit): ")?;
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

pub fn available_audio_player() -> io::Result<&'static str> {
    let paths = std::env::var_os("PATH").unwrap_or_default();

    ["pw-play", "paplay"]
        .iter()
        .copied()
        .find(|name| std::env::split_paths(&paths).any(|dir| dir.join(name).is_file()))
        .ok_or(Error::new(ErrorKind::NotFound, "no audio player found"))
}
