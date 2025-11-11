use anyhow::{bail, Context, Result};
use std::ffi::OsString;
use std::path::Path;

pub fn run(args: &[OsString]) -> Result<i32> {
    if args.is_empty() {
        bail!("mkdir: expected at least 1 argument, got 0");
    }

    let mut had_error = false;

    for arg in args {
        let path = Path::new(arg);

        if let Err(err) = std::fs::create_dir(path)
            .with_context(|| format!("mkdir: cannot create directory '{}'", path.display()))
        {
            eprintln!("{err}");
            had_error = true;
        }
    }

    Ok(if had_error { 1 } else { 0 })
}
