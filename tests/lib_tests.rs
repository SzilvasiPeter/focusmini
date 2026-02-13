use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

use focusmini::{available_audio_player, countdown, parse_args, parse_value, print_flush, run};

fn args<'a>(values: &'a [&'a str]) -> impl Iterator<Item = String> + 'a {
    values.iter().map(std::string::ToString::to_string)
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> io::Result<Self> {
        let path = std::env::temp_dir().join(name);
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir(&path)?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_dir_all(&self.path) {
            eprintln!(
                "warning: failed to remove temp dir '{}': {err}",
                self.path.display()
            );
        }
    }
}

#[test]
fn parse_args_defaults() {
    assert_eq!(parse_args(args(&[])).unwrap(), (60, 10));
}

#[test]
fn parse_args_custom_values() {
    assert_eq!(parse_args(args(&["-w", "25", "-b", "5"])).unwrap(), (25, 5));
}

#[test]
fn parse_args_missing_value_error() {
    assert_eq!(parse_args(args(&["-w"])).unwrap_err(), "no value for -w");
}

#[test]
fn parse_value_invalid_number() {
    assert_eq!(
        parse_value("--work", "abc").unwrap_err(),
        "invalid value 'abc' for --work"
    );
}

#[test]
fn parse_value_too_big_number() {
    assert_eq!(
        parse_value("--work", "1081").unwrap_err(),
        "--work value cannot exceed 1080 minutes"
    );
}

#[test]
fn parse_args_long_flags() {
    assert_eq!(
        parse_args(args(&["--work", "15", "--break", "7"])).unwrap(),
        (15, 7)
    );
}

#[test]
fn parse_args_ignores_unknown_flag() {
    assert_eq!(parse_args(args(&["--unknown", "1"])).unwrap(), (60, 10));
}

#[test]
fn parse_value_valid_number() {
    assert_eq!(parse_value("--break", "3").unwrap(), 3);
}

#[test]
fn countdown_zero_seconds_runs_quickly() {
    assert!(countdown("test", 0).is_ok());
}

struct ErrorNotifier;

impl focusmini::Notifier for ErrorNotifier {
    fn run(&self) -> io::Result<()> {
        Err(io::Error::other("not ready"))
    }
}

struct OkNotifier;

impl focusmini::Notifier for OkNotifier {
    fn run(&self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn run_propagates_notifier_error() {
    let mut input = Cursor::new(b"");
    let err = run(0, 0, &ErrorNotifier, &mut input).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::Other);
}

#[test]
fn run_stops_on_q_input() {
    let mut input = Cursor::new(b"q\n");
    assert!(run(0, 0, &OkNotifier, &mut input).is_ok());
}

#[test]
fn run_continues_after_enter_before_quit() {
    let mut input = Cursor::new(b"\nq\n");
    assert!(run(0, 0, &OkNotifier, &mut input).is_ok());
}

#[test]
fn countdown_one_second() {
    assert!(countdown("test", 1).is_ok());
}

#[cfg(feature = "fast-tick")]
#[test]
fn run_one_second_work_triggers_clear_line() {
    let mut input = Cursor::new(b"\nq\n");
    assert!(run(1, 0, &OkNotifier, &mut input).is_ok());
}

#[test]
fn print_flush_accepts_text() {
    assert!(print_flush("label").is_ok());
}

// Temporarily sets PATH while holding a global lock to avoid test interference,
// then restores the previous value.
#[allow(unsafe_code)]
fn with_path(path: &Path, f: impl FnOnce()) {
    static PATH_LOCK: Mutex<()> = Mutex::new(());
    let _guard = PATH_LOCK.lock().unwrap_or_else(PoisonError::into_inner);

    let prev = std::env::var_os("PATH");
    unsafe { std::env::set_var("PATH", path) };

    f();

    match prev {
        Some(val) => unsafe { std::env::set_var("PATH", val) },
        None => unsafe { std::env::remove_var("PATH") },
    }
}

#[test]
fn available_audio_player_returns_pw_play_when_present() {
    let dir = TempDir::new("available_audio_player_returns_pw_play_when_present").unwrap();
    std::fs::write(dir.path().join("pw-play"), b"").unwrap();
    with_path(dir.path(), || {
        assert_eq!(available_audio_player().unwrap(), "pw-play");
    });
}

#[test]
fn available_audio_player_returns_paplay_when_pw_play_missing() {
    let dir = TempDir::new("available_audio_player_returns_paplay_when_pw_play_missing").unwrap();
    std::fs::write(dir.path().join("paplay"), b"").unwrap();
    with_path(dir.path(), || {
        assert_eq!(available_audio_player().unwrap(), "paplay");
    });
}

#[test]
fn available_audio_player_errors_when_none_found() {
    let dir = TempDir::new("available_audio_player_errors_when_none_found").unwrap();
    with_path(dir.path(), || {
        assert_eq!(
            available_audio_player().unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
    });
}
