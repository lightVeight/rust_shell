use anyhow::{bail, Context};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    if args.len() != 2 {
        bail!("mv: expected 2 arguments (SRC DST), got {}", args.len());
    }
    let src = Path::new(&args[0]);
    let dst_input = Path::new(&args[1]);

    let src_meta = std::fs::symlink_metadata(src)
        .with_context(|| format!("mv: cannot stat '{}'", src.display()))?;

    let dst = resolve_dest_path(dst_input, src)?;

    match std::fs::rename(src, &dst) {
        Ok(_) => return Ok(0),
        Err(e) => {
            if !is_exdev(&e) {
                return Err(e).with_context(|| {
                    format!("mv: cannot move '{}' to '{}'", src.display(), dst.display())
                });
            }
        }
    }

    if src_meta.is_dir() {
        copy_dir_all(src, &dst).with_context(|| {
            format!(
                "mv: cannot copy dir '{}' to '{}'",
                src.display(),
                dst.display()
            )
        })?;
        std::fs::remove_dir_all(src)
            .with_context(|| format!("mv: cannot remove '{}'", src.display()))?;
    } else {
        std::fs::copy(src, &dst).with_context(|| {
            format!("mv: cannot copy '{}' to '{}'", src.display(), dst.display())
        })?;
        std::fs::remove_file(src)
            .with_context(|| format!("mv: cannot remove '{}'", src.display()))?;
    }

    Ok(0)
}

fn resolve_dest_path(dst_input: &Path, src: &Path) -> Result<PathBuf, anyhow::Error> {
    let is_dir = std::fs::symlink_metadata(dst_input)
        .map(|m| m.is_dir())
        .unwrap_or(false);
    if is_dir {
        let name = src.file_name().ok_or_else(|| {
            anyhow::anyhow!(format!("mv: invalid source name '{}'", src.display()))
        })?;
        Ok(dst_input.join(name))
    } else {
        Ok(dst_input.to_path_buf())
    }
}

fn is_exdev(err: &std::io::Error) -> bool {
    #[cfg(unix)]
    {
        matches!(err.raw_os_error(), Some(code) if code == libc::EXDEV)
    }
    #[cfg(not(unix))]
    {
        false
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            if ty.is_symlink() {
                #[cfg(unix)]
                {
                    let target = std::fs::read_link(&from)?;
                    std::os::unix::fs::symlink(&target, &to)?;
                }
                #[cfg(not(unix))]
                {
                    std::fs::copy(&from, &to)?;
                }
            } else {
                std::fs::copy(&from, &to)?;
            }
        }
    }
    Ok(())
}
