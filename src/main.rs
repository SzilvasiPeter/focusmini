#![forbid(unsafe_code)]
mod cli;

use cli::{parse_args, run};
use std::env::args;
use std::io::{Result, stdin};

const DEFAULT_SOUND: &str = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";

fn main() -> Result<()> {
    let (work, brk, sound) = parse_args(args().skip(1)).unwrap_or_else(|msg| {
        eprintln!("Argument warning: {msg}. Using default timers.");
        (60, 10, DEFAULT_SOUND.to_string())
    });
    run(work, brk, &sound, &mut stdin().lock())
}
