use anyhow::Context;

pub fn run(_args: &[std::ffi::OsString]) -> Result<i32, anyhow::Error> {
    let cwd = std::env::current_dir().context("failed to get current dir")?;
    println!("{}", cwd.display());
    Ok(0)
}