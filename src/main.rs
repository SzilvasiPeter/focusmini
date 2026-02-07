fn main() -> std::io::Result<()> {
    let work_minutes = 3;
    let break_minutes = 1;
    focusmini::run(work_minutes, break_minutes)
}
