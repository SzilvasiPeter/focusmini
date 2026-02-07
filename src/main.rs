fn main() -> std::io::Result<()> {
    match parse_args() {
        Ok((work_minutes, break_minutes)) => focusmini::run(work_minutes, break_minutes),
        Err(msg) => {
            eprintln!("Argument warning: {}. Using default timers.", msg);
            focusmini::run(60, 10)
        }
    }
}

fn parse_args() -> Result<(u16, u16), String> {
    let mut work = 60;
    let mut rest = 10;
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    for chunk in args.chunks(2) {
        let (flag, value) = match chunk {
            [flag, value] => (flag, value),
            [flag] => return Err(format!("missing value for {}", flag)),
            _ => continue,
        };
        match flag.as_str() {
            "-w" | "--work" => {
                let parsed = match value.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("invalid value for {}", flag)),
                };
                work = parsed;
            }
            "-b" | "--break" => {
                let parsed = match value.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("invalid value for {}", flag)),
                };
                rest = parsed;
            }
            _ => {}
        }
    }

    Ok((work, rest))
}
