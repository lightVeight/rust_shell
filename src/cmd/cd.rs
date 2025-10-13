use std::{env, ffi::OsString};
use anyhow::{Context, bail};

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let target = if args.is_empty() {
        env::var_os("HOME").ok_or_else(|| anyhow::anyhow!("cd: HOME not set"))?
    } else if args.len() == 1 {
        args[0].clone()
    } else {
        bail!("cd: expected 0 or 1 argument, got {}", args.len());
    };

    env::set_current_dir(&target).with_context(|| {
        format!("cd: could not change to '{}'", target.to_string_lossy())
    })?;
    Ok(0)
}