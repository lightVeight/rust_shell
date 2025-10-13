use std::ffi::OsString;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let line = args
        .iter()
        .map(|s| s.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{line}");
    Ok(0)
}
