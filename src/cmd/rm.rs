use anyhow::{Context, bail};
use std::ffi::OsString;
use std::path::Path;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    if args.len() != 1 {
        bail!("rm: expected 1 argument, got {}", args.len());
    }
    let path = Path::new(&args[0]);

    std::fs::remove_file(path)
        .with_context(|| format!("rm: cannot remove '{}'", path.display()))?;

    Ok(0)
}
