use anyhow::{Context, bail};
use std::ffi::OsString;
use std::path::Path;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    if args.len() != 1 {
        bail!("mkdir: expected 1 argument, got {}", args.len());
    }
    let path = Path::new(&args[0]);

    std::fs::create_dir(path)
        .with_context(|| format!("mkdir: cannot create directory '{}'", path.display()))?;

    Ok(0)
}
