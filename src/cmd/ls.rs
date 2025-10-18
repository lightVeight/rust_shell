use anyhow::{Context, bail};
use std::ffi::OsString;
use std::path::Path;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let mut show_all = false; // -a
    let mut argu: Option<&OsString> = None;

    for a in args {
        if a == "-a" {
            show_all = true;
        } else if argu.is_none() {
            argu = Some(a);
        } else {
            bail!("ls: too many arguments");
        }
    }

    let target = if let Some(first) = argu {
        Path::new(first)
    } else {
        Path::new(".")
    };

    let meta = std::fs::symlink_metadata(target)
        .with_context(|| format!("ls: cannot access '{}'", target.display()))?;

    if meta.is_file() || meta.file_type().is_symlink() {
        println!("{}", target.display());
        return Ok(0);
    }

    if meta.is_dir() {
        let mut names = Vec::new();
        for entry in std::fs::read_dir(target)
            .with_context(|| format!("ls: cannot open directory '{}'", target.display()))?
        {
            let entry = entry?;
            let name = entry.file_name();

            // skip dotfiles only if no -a
            if !show_all && name.to_string_lossy().starts_with('.') {
                continue;
            }
            names.push(name);
        }

        names.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));
        for name in names {
            print!("{}  ", name.to_string_lossy());
        }
        println!();
        return Ok(0);
    }

    println!("{}", target.display());
    Ok(0)
}
