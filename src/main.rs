use clap::Parser;
use rodio::{OutputStreamBuilder, Sink, Source, source::SineWave};
use std::io::{self, Write, stdin, stdout};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about)]
struct Config {
    #[arg(short = 'w', long = "work", default_value_t = 60)]
    work_minutes: u64,
    #[arg(short = 'b', long = "break", default_value_t = 10)]
    break_minutes: u64,
}

fn main() -> anyhow::Result<()> {
    let config = Config::parse();
    let work_seconds = config.work_minutes * 60;
    let break_seconds = config.break_minutes * 60;
    let work = ("\x1b[1m [Work] \x1b[0m", work_seconds);
    let pause = ("\x1b[1m [Break] \x1b[0m", break_seconds);

    loop {
        for (label, seconds) in [work, pause] {
            countdown(label, seconds)?;
            beep()?;
            if !confirm_continue()? {
                return Ok(());
            }
        }
    }
}

fn countdown(label: &str, seconds: u64) -> io::Result<()> {
    for remaining in (1..=seconds).rev() {
        let timer = format!("{label} {:02}:{:02}", remaining / 60, remaining % 60);
        reset_line(&timer)?;
        sleep(Duration::from_secs(1));
    }

    let finished = format!("{label} 00:00");
    reset_line(&finished)?;

    Ok(())
}

fn beep() -> anyhow::Result<(), rodio::StreamError> {
    let mut stream = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(stream.mixer());

    sink.append(SineWave::new(800.0).take_duration(Duration::from_millis(300)));
    sink.append(SineWave::new(1.0).take_duration(Duration::from_millis(300)));
    sink.append(SineWave::new(800.0).take_duration(Duration::from_millis(300)));

    sink.sleep_until_end();
    stream.log_on_drop(false);

    Ok(())
}

fn confirm_continue() -> io::Result<bool> {
    loop {
        reset_line("Enter `n` to continue or `q` to exit: ")?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let clear_query = format!("\x1B[1A\r{:<80}\r", "");
        reset_line(&clear_query)?;

        match input.chars().next() {
            Some('n') => return Ok(true),
            Some('q') => return Ok(false),
            _ => {}
        }
    }
}

fn reset_line(text: &str) -> io::Result<()> {
    print!("\r{text}");
    stdout().flush()?;
    Ok(())
}
