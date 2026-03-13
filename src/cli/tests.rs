use crate::DEFAULT_SOUND;

use super::{countdown, parse_args, parse_value, print_flush, run};
use std::io::Cursor;

fn args<'a>(values: &'a [&'a str]) -> impl Iterator<Item = String> + 'a {
    values.iter().map(std::string::ToString::to_string)
}

#[test]
fn parse_args_defaults() {
    assert_eq!(
        parse_args(args(&[])).unwrap(),
        (60, 10, DEFAULT_SOUND.to_string())
    );
}

#[test]
fn parse_args_custom_values() {
    assert_eq!(
        parse_args(args(&["-w", "25", "-b", "5"])).unwrap(),
        (25, 5, DEFAULT_SOUND.to_string())
    );
}

#[test]
fn parse_args_missing_value_error() {
    assert_eq!(parse_args(args(&["-w"])).unwrap_err(), "no value for -w");
}

fn arg(value: &str) -> impl Iterator<Item = String> + '_ {
    std::iter::once(value).map(std::string::ToString::to_string)
}

#[test]
fn parse_value_invalid_number() {
    let mut values = arg("abc");
    assert_eq!(
        parse_value("--work", &mut values).unwrap_err(),
        "invalid value 'abc' for --work"
    );
}

#[test]
fn parse_value_too_big_number() {
    let mut values = arg("1081");
    assert_eq!(
        parse_value("--work", &mut values).unwrap_err(),
        "--work value should be between 1 and 1080 minutes"
    );
}

#[test]
fn parse_args_long_flags() {
    assert_eq!(
        parse_args(args(&["--work", "15", "--break", "7"])).unwrap(),
        (15, 7, DEFAULT_SOUND.to_string())
    );
}

#[test]
fn parse_args_sound_flag() {
    assert_eq!(
        parse_args(args(&["-s", "/tmp/alert.oga"])).unwrap(),
        (60, 10, "/tmp/alert.oga".to_string())
    );
}

#[test]
fn parse_args_unknown_flag_error() {
    assert_eq!(
        parse_args(args(&["--unknown", "1"])).unwrap_err(),
        "unknown flag --unknown"
    );
}

#[test]
fn parse_value_valid_number() {
    let mut values = arg("3");
    assert_eq!(parse_value("--break", &mut values).unwrap(), 3);
}

#[test]
fn countdown_zero_seconds_runs_quickly() {
    assert!(countdown("test", 0).is_ok());
}

#[test]
fn countdown_one_second() {
    assert!(countdown("test", 1).is_ok());
}

#[test]
fn run_zero_second_no_countdown() {
    let mut input = Cursor::new(b"\nq\n");
    assert!(run(0, 0, "none", &mut input).is_ok());
}

#[test]
fn run_one_second_print_countdown() {
    let mut input = Cursor::new(b"\nq\n");
    assert!(run(1, 0, "none", &mut input).is_ok());
}

#[test]
fn print_flush_accepts_text() {
    assert!(print_flush("label").is_ok());
}
