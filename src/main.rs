#![forbid(unsafe_code)]
// TODO: Remove this once the releated issues are resolved:
// - https://github.com/jni-rs/jni-rs/issues/806
// - https://github.com/rust-mobile/ndk/issues/514
#![allow(clippy::multiple_crate_versions)]
mod cli;

use cli::{SoundPlayer, parse_args, run};
use rodio::source::SquareWave;
use rodio::{DeviceSinkBuilder, Source};
use std::env::args;
use std::io::{Error, Result, stdin};
use std::time::Duration;

struct RodioSoundPlayer;

impl SoundPlayer for RodioSoundPlayer {
    type Guard = rodio::MixerDeviceSink;

    fn play(&self) -> Result<Self::Guard> {
        let mut stream = DeviceSinkBuilder::open_default_sink().map_err(Error::other)?;
        stream.log_on_drop(false);
        let mixer = stream.mixer();

        let beep = Duration::from_millis(120);
        let gap = Duration::from_millis(80);
        let pause = Duration::from_millis(350);
        let step = beep + gap;

        let beep = |delay: Duration| {
            SquareWave::new(880.0)
                .amplify(0.3)
                .take_duration(beep)
                .delay(delay)
        };

        mixer.add(beep(Duration::ZERO));
        mixer.add(beep(step));
        mixer.add(beep(step + step + pause));
        mixer.add(beep(step + step + pause + step));

        Ok(stream)
    }
}

fn main() -> Result<()> {
    let (work, brk) = parse_args(args().skip(1)).unwrap_or_else(|msg| {
        eprintln!("Argument warning: {msg}. Using default timers.");
        (60, 10)
    });
    let stdin = stdin();
    let mut stdin_lock = stdin.lock();
    run(work, brk, &mut stdin_lock, &RodioSoundPlayer)
}
