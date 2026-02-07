#![forbid(unsafe_code)]
use std::io::{self, Write, stdin, stdout};

pub fn parse_args(mut args: std::env::Args) -> Result<(u16, u16), String> {
    args.next();
    let mut work = 60;
    let mut rest = 10;

    while let Some(flag) = args.next() {
        let error = format!("missing value for {}", flag);
        let value = args.next().ok_or(error)?;
        match flag.as_str() {
            "-w" | "--work" => work = parse_value(flag.as_str(), value.as_str())?,
            "-b" | "--break" => rest = parse_value(flag.as_str(), value.as_str())?,
            _ => {}
        }
    }

    Ok((work, rest))
}

pub fn parse_value(flag: &str, value: &str) -> Result<u16, String> {
    let error = format!("invalid value for {}", flag);
    value.parse::<u16>().map_err(|_| error)
}

pub fn run(work_minutes: u16, break_minutes: u16) -> io::Result<()> {
    let work_seconds = work_minutes * 60;
    let break_seconds = break_minutes * 60;
    let work = ("\x1b[1m [Work] \x1b[0m", work_seconds);
    let pause = ("\x1b[1m [Break] \x1b[0m", break_seconds);

    for (label, seconds) in [work, pause].into_iter().cycle() {
        countdown(label, seconds)?;
        notify()?;

        let mut input = String::new();
        print_flush("\rEnter to continue (q to quit): ")?;
        stdin().read_line(&mut input)?;
        if matches!(input.chars().next(), Some('q')) {
            break;
        }

        print_flush("\x1B[1A\x1B[2K")?; // move up and clear line
    }

    Ok(())
}

pub fn countdown(label: &str, seconds: u16) -> io::Result<()> {
    for sec in (1..=seconds).rev() {
        print_flush(&format!("\r{label} {:02}:{:02}", sec / 60, sec % 60))?;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    print_flush(&format!("\r{label} 00:00"))?;
    Ok(())
}

pub fn notify() -> io::Result<()> {
    let alarm = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";
    let status = std::process::Command::new("paplay").arg(alarm).status()?;
    let err = io::Error::other("paplay failed");
    status.success().then_some(()).ok_or(err)
}

pub fn print_flush(text: &str) -> io::Result<()> {
    print!("{text}");
    stdout().flush()?;
    Ok(())
}
