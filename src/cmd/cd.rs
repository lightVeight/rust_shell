use anyhow::{bail, Context};
use std::{env, ffi::OsString, path::Path};

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let requested = match args.len() {
        0 => None, //HOME
        1 => Some(&args[0]),
        _ => bail!("cd: expected 0 or 1 argument, got {}", args.len()),
    };

    let target_os: OsString = match requested {
        None => env::var_os("HOME").ok_or_else(|| anyhow::anyhow!("cd: HOME not set"))?,
        Some(s) if s == "-" => {
            env::var_os("OLDPWD").ok_or_else(|| anyhow::anyhow!("cd: OLDPWD not set"))?
        }
        Some(s) => s.clone(),
    };

    let cur_dir = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());

    env::set_current_dir(&target_os)
        .with_context(|| format!("cd: could not change to '{}'", target_os.to_string_lossy()))?;

    env::set_var("OLDPWD", cur_dir.to_string_lossy().to_string());
    if let Ok(newpwd) = env::current_dir() {
        env::set_var("PWD", newpwd.to_string_lossy().to_string());
    }

    if requested.map(|s| s == "-").unwrap_or(false) {
        if let Ok(cur) = env::current_dir() {
            println!("{}", cur.display());
        }
    }

    Ok(0)
}
