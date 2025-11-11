use anyhow::{bail, Context};
use std::ffi::OsString;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    if !args.is_empty() {
        bail!("clear: expected 0 arguments, got {}", args.len());
    }

    print!("\x1B[2J\x1B[H");
    std::io::Write::flush(&mut std::io::stdout()).context("clear: failed to flush stdout")?;

    Ok(0)
}
