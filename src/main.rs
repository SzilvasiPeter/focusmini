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
    let mut args = std::env::args().skip(1);
    let parse_value = |flag: &str, value: &str| {
        let error = format!("invalid value for {}", flag);
        value.parse::<u16>().map_err(|_| error)
    };

    while let Some(flag) = args.next() {
        let error = format!("missing value for {}", flag);
        let value = args.next().ok_or_else(|| error)?;
        match flag.as_str() {
            "-w" | "--work" => work = parse_value(&flag, &value)?,
            "-b" | "--break" => rest = parse_value(&flag, &value)?,
            _ => {}
        }
    }

    Ok((work, rest))
}
