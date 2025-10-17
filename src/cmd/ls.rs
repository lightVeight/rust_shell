use anyhow::Context;
use std::ffi::OsString;
use std::path::Path;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let target = if let Some(first) = args.first() {
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

            // skip dotfiles
            if name.to_string_lossy().starts_with('.') {
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
