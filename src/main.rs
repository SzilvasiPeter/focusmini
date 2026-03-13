#![forbid(unsafe_code)]
mod cli;

use cli::{parse_args, run};
use std::env::args;
use std::io::{Result, stdin};

fn main() -> Result<()> {
    let (work, brk) = parse_args(args().skip(1)).unwrap_or_else(|msg| {
        eprintln!("Argument warning: {msg}. Using default timers.");
        (60, 10)
    });
    run(work, brk, &mut stdin().lock())
}
