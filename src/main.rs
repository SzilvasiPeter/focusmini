use clap::Parser;
use focusmini::{Config, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    run(config)
}
