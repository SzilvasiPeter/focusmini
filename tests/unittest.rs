use focusmini::{countdown, print_flush};

#[test]
fn countdown_zero_minutes_is_ok() {
    assert!(countdown("[Test]", 0).is_ok());
}

#[test]
fn print_flush_accepts_text() {
    assert!(print_flush("output").is_ok());
}
