#![forbid(unsafe_code)]

pub struct APlayer(&'static str);

impl focusmini::Notifier for APlayer {
    fn run(&self) -> std::io::Result<()> {
        std::process::Command::new(self.0)
            .arg("/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga")
            .status()?
            .success()
            .then_some(())
            .ok_or(std::io::Error::other(format!("{} failed", self.0)))
    }
}

fn main() -> std::io::Result<()> {
    let (work, brk) = focusmini::parse_args(std::env::args()).unwrap_or_else(|msg| {
        eprintln!("Argument warning: {}. Using default timers.", msg);
        (60, 10)
    });
    let player = focusmini::available_audio_player()?;
    focusmini::run(work, brk, &APlayer(player), &mut std::io::stdin().lock())
}
