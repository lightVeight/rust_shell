use anyhow::{Context, bail};
use std::ffi::OsString;
use std::path::Path;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    if args.len() != 1 {
        bail!("rm: expected 1 argument, got {}", args.len());
    }

    let path = Path::new(&args[0]);
    let meta = std::fs::symlink_metadata(path)
        .with_context(|| format!("rm: cannot access '{}'", path.display()))?;

    if meta.file_type().is_dir() {
        bail!("rm: cannot remove '{}': Is a directory", path.display());
    }

    std::fs::remove_file(path)
        .with_context(|| format!("rm: cannot remove '{}'", path.display()))?;

    Ok(0)
}
