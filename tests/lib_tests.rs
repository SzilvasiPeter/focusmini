#![forbid(unsafe_code)]

use focusmini::{countdown, parse_args, parse_value};

fn args<'a>(values: &'a [&'a str]) -> impl Iterator<Item = String> + 'a {
    values.iter().map(|v| v.to_string())
}

#[test]
fn parse_args_defaults() {
    assert_eq!(parse_args(args(&["prog"])).unwrap(), (60, 10));
}

#[test]
fn parse_args_custom_values() {
    assert_eq!(
        parse_args(args(&["prog", "-w", "25", "-b", "5"])).unwrap(),
        (25, 5)
    );
}

#[test]
fn parse_args_missing_value_error() {
    assert_eq!(
        parse_args(args(&["prog", "-w"])).unwrap_err(),
        "missing value for -w"
    );
}

#[test]
fn parse_value_invalid_number() {
    assert_eq!(
        parse_value("--work", "abc").unwrap_err(),
        "invalid value for --work"
    );
}

#[test]
fn parse_args_long_flags() {
    assert_eq!(
        parse_args(args(&["prog", "--work", "15", "--break", "7"])).unwrap(),
        (15, 7)
    );
}

#[test]
fn parse_args_ignores_unknown_flag() {
    assert_eq!(
        parse_args(args(&["prog", "--unknown", "1"])).unwrap(),
        (60, 10)
    );
}

#[test]
fn parse_value_valid_number() {
    assert_eq!(parse_value("--break", "3").unwrap(), 3);
}

#[test]
fn countdown_zero_seconds_runs_quickly() {
    assert!(countdown("test", 0).is_ok());
}
