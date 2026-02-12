use focusmini::{Notifier, available_audio_player, parse_args, run};
use std::env::args;
use std::io::{Error, Result, stdin};
use std::process::Command;

pub struct APlayer(&'static str);

impl Notifier for APlayer {
    fn run(&self) -> Result<()> {
        Command::new(self.0)
            .arg("/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga")
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
    let player = available_audio_player()?;
    run(work, brk, &APlayer(player), &mut stdin().lock())
}
