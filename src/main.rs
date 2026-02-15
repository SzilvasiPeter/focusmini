#![forbid(unsafe_code)]
mod cli;

use cli::{Notifier, find_audio_player, parse_args, run};
use std::env::{args, var_os};
use std::io::{Error, ErrorKind, Result, stdin};
use std::path::Path;
use std::process::Command;

const DEFAULT_SOUND: &str = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";

pub struct APlayer {
    player: &'static str,
    sound: &'static str,
}

impl Notifier for APlayer {
    fn run(&self) -> Result<()> {
        if !Path::new(self.sound).is_file() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("missing: {}", self.sound),
            ));
        }
        Command::new(self.player)
            .arg(self.sound)
            .status()?
            .success()
            .then_some(())
            .ok_or_else(|| Error::other(format!("{} failed", self.player)))
    }
}

fn main() -> Result<()> {
    let (work, brk) = parse_args(args().skip(1)).unwrap_or_else(|msg| {
        eprintln!("Argument warning: {msg}. Using default timers.");
        (60, 10)
    });
    let paths = var_os("PATH").ok_or_else(|| Error::new(ErrorKind::NotFound, "no PATH env"))?;
    let player = find_audio_player(&paths)?;
    let alarm = APlayer {
        player,
        sound: DEFAULT_SOUND,
    };
    run(work, brk, &alarm, &mut stdin().lock())
}

#[cfg(test)]
mod tests {
    use super::{APlayer, Notifier};
    use std::io::ErrorKind;

    #[test]
    fn missing_sound_returns_not_found() {
        let alarm = APlayer {
            player: "pw-play",
            sound: "/tmp/focusmini-missing-sound.oga",
        };
        let err = alarm.run().expect_err("expected missing sound");
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert_eq!(err.to_string(), "missing: /tmp/focusmini-missing-sound.oga");
    }
}
