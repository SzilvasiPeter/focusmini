use clap::Parser;
use focusmini::Config;

#[test]
fn default_config_matches_previous_cli() {
    let config = Config::parse_from(["focusmini"]);

    assert_eq!(config.work_minutes, 60);
    assert_eq!(config.break_minutes, 10);
    assert_eq!(config.amplify, 1.0);
}

#[test]
fn short_options_remain_backwards_compatible() {
    let config = Config::parse_from(["focusmini", "-w", "25", "-b", "5", "-a", "2.5"]);

    assert_eq!(config.work_minutes, 25);
    assert_eq!(config.break_minutes, 5);
    assert_eq!(config.amplify, 2.5);
}

#[test]
fn long_option_names_still_parse() {
    let config = Config::parse_from([
        "focusmini",
        "--work",
        "45",
        "--break",
        "15",
        "--amplify",
        "0.5",
    ]);

    assert_eq!(config.work_minutes, 45);
    assert_eq!(config.break_minutes, 15);
    assert_eq!(config.amplify, 0.5);
}
