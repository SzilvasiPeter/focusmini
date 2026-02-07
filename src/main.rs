fn main() -> std::io::Result<()> {
    match focusmini::parse_args(std::env::args().skip(1)) {
        Ok((work_minutes, break_minutes)) => focusmini::run(work_minutes, break_minutes),
        Err(msg) => {
            eprintln!("Argument warning: {}. Using default timers.", msg);
            focusmini::run(60, 10)
        }
    }
}
