use anyhow::{bail, Context};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let mut recursive = false;
    let mut rest: Vec<&OsString> = Vec::new();
    for a in args {
        if a == "-r" {
            recursive = true;
        } else {
            rest.push(a);
        }
    }

    match rest.len() {
        0 => bail!("cp: missing file operand"),
        1 => bail!(
            "cp: missing destination file operand after '{}'",
            Path::new(rest[0]).display()
        ),
        2 => {}
        _ => bail!("cp: extra operand '{}'", Path::new(rest[2]).display()),
    }

    let src = Path::new(rest[0]);
    let dst_input = Path::new(rest[1]);

    let src_meta = fs::symlink_metadata(src)
        .with_context(|| format!("cp: cannot stat '{}'", src.display()))?;

    if src_meta.is_dir() {
        if !recursive {
            bail!(
                "cp: -r not specified; omitting directory '{}'",
                src.display()
            );
        }
        copy_dir_entry(src, dst_input)?;
    } else {
        let dst = resolve_dest_path(dst_input, src)?;
        copy_file_overwrite(src, &dst)?;
    }

    Ok(0)
}

fn resolve_dest_path(dst_input: &Path, src: &Path) -> Result<PathBuf, anyhow::Error> {
    match fs::symlink_metadata(dst_input) {
        Ok(meta) if meta.is_dir() => {
            let name = src.file_name().ok_or_else(|| {
                anyhow::anyhow!(format!("cp: invalid source name '{}'", src.display()))
            })?;
            Ok(dst_input.join(name))
        }
        _ => Ok(dst_input.to_path_buf()),
    }
}

fn copy_file_overwrite(src: &Path, dst: &Path) -> Result<(), anyhow::Error> {
    if let Some(parent) = dst.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("cp: cannot create directory '{}'", parent.display()))?;
        }
    }

    fs::copy(src, dst)
        .with_context(|| format!("cp: cannot copy '{}' to '{}'", src.display(), dst.display()))?;

    if let Ok(meta) = fs::symlink_metadata(src) {
        let _ = fs::set_permissions(dst, meta.permissions());
    }

    Ok(())
}

fn copy_dir_entry(src_dir: &Path, dst_input: &Path) -> Result<(), anyhow::Error> {
    let dst_final = match fs::symlink_metadata(dst_input) {
        Ok(meta) => {
            if meta.is_dir() {
                let base = src_dir.file_name().ok_or_else(|| {
                    anyhow::anyhow!(format!(
                        "cp: invalid source directory '{}'",
                        src_dir.display()
                    ))
                })?;
                dst_input.join(base)
            } else {
                bail!(
                    "cp: cannot overwrite non-directory '{}' with directory '{}'",
                    dst_input.display(),
                    src_dir.display()
                );
            }
        }
        Err(_) => dst_input.to_path_buf(),
    };

    copy_dir_recursive(src_dir, &dst_final)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), anyhow::Error> {
    fs::create_dir_all(dst).with_context(|| {
        format!(
            "cp: cannot create directory '{}' for copying '{}'",
            dst.display(),
            src.display()
        )
    })?;

    if let Ok(meta) = fs::symlink_metadata(src) {
        let _ = fs::set_permissions(dst, meta.permissions());
    }

    for entry in fs::read_dir(src)
        .with_context(|| format!("cp: cannot read directory '{}'", src.display()))?
    {
        let entry = entry?;
        let sp = entry.path();
        let dp = dst.join(entry.file_name());

        let meta = match fs::metadata(&sp) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("cp: cannot stat '{}': {}", sp.display(), e);
                continue;
            }
        };

        if meta.is_dir() {
            copy_dir_recursive(&sp, &dp)?;
        } else {
            copy_file_overwrite(&sp, &dp)?;
        }
    }

    Ok(())
}
