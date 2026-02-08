#![forbid(unsafe_code)]
pub struct Paplay;

impl focusmini::Notifier for Paplay {
    fn run(&self) -> std::io::Result<()> {
        let alarm = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";
        let status = std::process::Command::new("paplay").arg(alarm).status()?;
        let err = std::io::Error::other("paplay failed");
        status.success().then_some(()).ok_or(err)
    }
}

fn main() -> std::io::Result<()> {
    let mut locked = std::io::stdin().lock();
    match focusmini::parse_args(std::env::args()) {
        Ok((work, brk)) => focusmini::run(work, brk, &Paplay, &mut locked),
        Err(msg) => {
            eprintln!("Argument warning: {}. Using default timers.", msg);
            focusmini::run(60, 10, &Paplay, &mut locked)
        }
    }
}
