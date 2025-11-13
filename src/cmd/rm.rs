use anyhow::bail;
use std::ffi::OsString;
use std::path::Path;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    if args.is_empty() {
        bail!("rm: missing operand");
    }

    let mut recursive = false;
    let mut paths: Vec<OsString> = Vec::new();
    let mut stop_flags = false;

    for a in args {
        if !stop_flags {
            if a == "--" {
                stop_flags = true;
                continue;
            }
            if let Some(s) = a.to_str() {
                if s.starts_with('-') && s != "-" {
                    for ch in s.chars().skip(1) {
                        match ch {
                            'r' | 'R' => {
                                recursive = true;
                            }
                            _ => bail!(
                                "rm: invalid option -- '{}'\n\
                                 hint: use '--' before file names that start with '-'",
                                ch
                            ),
                        }
                    }
                    continue;
                }
            }
        }
        paths.push(a.clone());
    }

    if paths.is_empty() {
        bail!("rm: missing operand");
    }

    let mut exit_code = 0;

    for a in paths {
        let path = Path::new(&a);

        let meta = match std::fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}", path.display(), e);
                exit_code = 1;
                continue;
            }
        };

        let ft = meta.file_type();
        if ft.is_dir() {
            if recursive {
                if let Err(e) = std::fs::remove_dir_all(path) {
                    eprintln!("rm: cannot remove '{}': {}", path.display(), e);
                    exit_code = 1;
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", path.display());
                exit_code = 1;
            }
        } else {
            if let Err(e) = std::fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}", path.display(), e);
                exit_code = 1;
            }
        }
    }

    Ok(exit_code)
}
