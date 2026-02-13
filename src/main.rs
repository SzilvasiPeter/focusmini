#![forbid(unsafe_code)]
mod cli;

use cli::{Notifier, find_audio_player, parse_args, run};
use std::env::{args, var_os};
use std::io::{Error, ErrorKind, Result, stdin};
use std::path::Path;
use std::process::Command;

pub struct APlayer(&'static str);

impl Notifier for APlayer {
    fn run(&self) -> Result<()> {
        let sound = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";
        if !Path::new(sound).is_file() {
            return Err(Error::new(ErrorKind::NotFound, format!("missing: {sound}")));
        }
        Command::new(self.0)
            .arg(sound)
            .status()?
            .success()
            .then_some(())
            .ok_or_else(|| Error::other(format!("{} failed", self.0)))
    }
}

fn main() -> Result<()> {
    let (work, brk) = parse_args(args().skip(1)).unwrap_or_else(|msg| {
        eprintln!("Argument warning: {msg}. Using default timers.");
        (60, 10)
    });
    let paths = var_os("PATH").ok_or_else(|| Error::new(ErrorKind::NotFound, "no PATH env"))?;
    let player = find_audio_player(&paths)?;
    run(work, brk, &APlayer(player), &mut stdin().lock())
}
