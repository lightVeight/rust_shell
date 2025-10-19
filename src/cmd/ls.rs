use anyhow::{Context, bail};
use std::ffi::OsString;
use std::path::Path;

#[cfg(unix)]
fn classify_suffix(ft: &std::fs::FileType, meta: &std::fs::Metadata) -> &'static str {
    use std::os::unix::fs::PermissionsExt;
    if ft.is_symlink() {
        "@"
    } else if ft.is_dir() {
        "/"
    } else {
        //regular file
        let mode = meta.permissions().mode();
        //if any of owner/group/others executable bit is set means '*'
        if mode & 0o111 != 0 { "*" } else { "" }
    }
}

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let mut show_all = false; // -a
    let mut classify = false; // -F
    let mut argu: Option<&OsString> = None;

    for a in args {
        if a == "-a" {
            show_all = true;
        } else if a == "-F" {
            classify = true;
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
        if classify {
            let ft = meta.file_type();
            let suffix = classify_suffix(&ft, &meta);
            println!("{}{}", target.display(), suffix);
        } else {
            println!("{}", target.display());
        }
        return Ok(0);
    }

    if meta.is_dir() {
        let mut labeled: Vec<String> = Vec::new();

        for entry in std::fs::read_dir(target)
            .with_context(|| format!("ls: cannot open directory '{}'", target.display()))?
        {
            let entry = entry?;
            let name = entry.file_name();

            // skip dotfiles only if no -a
            if !show_all && name.to_string_lossy().starts_with('.') {
                continue;
            }

            if classify {
                let entry_meta = std::fs::symlink_metadata(entry.path())?;
                let ft = entry_meta.file_type();
                let suffix = classify_suffix(&ft, &entry_meta);
                labeled.push(format!("{}{}", name.to_string_lossy(), suffix));
            } else {
                labeled.push(name.to_string_lossy().into_owned());
            }
        }

        labeled.sort();
        for s in labeled {
            print!("{}  ", s);
        }
        println!();
        return Ok(0);
    }

    println!("{}", target.display());
    Ok(0)
}
