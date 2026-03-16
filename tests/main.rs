use std::io::Write;
use std::process::{Command, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_focusmini"))
}

#[test]
fn happy_path_valid_args() {
    let mut cmd = bin();
    let _ = cmd
        .arg("--work")
        .arg("1")
        .arg("--break")
        .arg("1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("spawn focusmini");
    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin.write_all(b"q\n").expect("write stdin");
    }

    let out = child.wait_with_output().expect("wait");
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!stderr.contains("Argument warning"));
}
