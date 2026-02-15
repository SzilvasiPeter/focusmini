use std::env::temp_dir;
use std::fs::{File, create_dir, remove_dir, set_permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn run(
    args: &[&str],
    path: Option<&std::path::Path>,
    remove_path: bool,
    input: Option<&str>,
) -> (bool, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_focusmini"));
    let _ = cmd.args(args);
    if remove_path {
        let _ = cmd.env_remove("PATH");
    }
    if let Some(p) = path {
        let _ = cmd.env("PATH", p);
    }
    if let Some(text) = input {
        let _ = cmd.stdin(Stdio::piped());
        let _ = cmd.stdout(Stdio::null());
        let mut child = cmd.spawn().expect("binary should start");
        let mut input_pipe = child.stdin.take().expect("stdin should be piped");
        input_pipe
            .write_all(text.as_bytes())
            .expect("stdin write should succeed");
        drop(input_pipe);
        let out = child.wait_with_output().expect("binary should exit");
        return (
            !out.status.success(),
            String::from_utf8_lossy(&out.stderr).into_owned(),
        );
    }
    let out = cmd.output().expect("binary should run");
    (
        !out.status.success(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn unique_dir(prefix: &str) -> std::path::PathBuf {
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    temp_dir().join(format!("{prefix}-{ns}"))
}

fn make_player(path: &std::path::Path) {
    let player = path.join("pw-play");
    let mut file = File::create(&player).expect("player should be created");
    file.write_all(b"#!/bin/sh\nexit 0\n")
        .expect("player script should be written");
    let perm = std::fs::Permissions::from_mode(0o755);
    set_permissions(player, perm).expect("player should be executable");
}

#[test]
fn fails_when_path_is_missing() {
    let (failed, err) = run(&[], None, true, None);

    assert!(failed);
    assert!(err.contains("no PATH env"), "stderr was: {err}");
}

#[test]
fn warns_on_bad_args_then_uses_defaults() {
    let (failed, err) = run(&["--bad"], None, true, None);

    assert!(failed);
    assert!(err.contains("Argument warning:"), "stderr was: {err}");
    assert!(err.contains("unknown flag --bad"), "stderr was: {err}");
    assert!(err.contains("no PATH env"), "stderr was: {err}");
}

#[test]
fn fails_when_no_audio_player_is_found() {
    let path = unique_dir("focusmini-it");
    create_dir(&path).expect("temp test dir should be created");

    let (failed, err) = run(&[], Some(&path), false, None);

    let _ = remove_dir(&path);

    assert!(failed);
    assert!(err.contains("no audio player found"), "stderr was: {err}");
}

#[test]
fn succeeds_on_happy_path() {
    let sound = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";
    if !std::path::Path::new(sound).is_file() {
        return;
    }

    let path = unique_dir("focusmini-it-ok");
    create_dir(&path).expect("temp test dir should be created");
    make_player(&path);

    let (failed, _) = run(&["-w", "0", "-b", "0"], Some(&path), false, Some("q\n"));

    let _ = remove_dir(&path);
    assert!(!failed);
}
