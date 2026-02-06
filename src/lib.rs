#![forbid(unsafe_code)]

use clap::Parser;
use rodio::{OutputStreamBuilder, Sink, Source, StreamError, source::SineWave};
use std::io::{self, Write, stdin, stdout};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(short = 'w', long = "work", default_value_t = 60)]
    pub work_minutes: u16,
    #[arg(short = 'b', long = "break", default_value_t = 10)]
    pub break_minutes: u16,
    #[arg(short = 'a', long = "amplify", default_value_t = 1.0)]
    pub amplify: f32,
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let work_seconds = config.work_minutes * 60;
    let break_seconds = config.break_minutes * 60;
    let work = ("\x1b[1m [Work] \x1b[0m", work_seconds);
    let pause = ("\x1b[1m [Break] \x1b[0m", break_seconds);

    for (label, seconds) in [work, pause].into_iter().cycle() {
        countdown(label, seconds)?;
        beep(config.amplify)?;

        let mut input = String::new();
        print_flush("\rPress Enter to continue or type `q` to quit: ")?;
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
        std::thread::sleep(Duration::from_secs(1));
    }

    print_flush(&format!("\r{label} 00:00"))?;
    Ok(())
}

pub fn beep(volume: f32) -> Result<(), StreamError> {
    let mut stream = OutputStreamBuilder::open_default_stream()?;
    stream.log_on_drop(false);

    let sink = Sink::connect_new(stream.mixer());
    let duration = Duration::from_millis(300);
    sink.append(SineWave::new(500.0).amplify(volume).take_duration(duration));
    sink.append(SineWave::new(1.0).amplify(volume).take_duration(duration));
    sink.append(SineWave::new(500.0).amplify(volume).take_duration(duration));
    sink.sleep_until_end();
    Ok(())
}

pub fn print_flush(text: &str) -> io::Result<()> {
    print!("{text}");
    stdout().flush()?;
    Ok(())
}
