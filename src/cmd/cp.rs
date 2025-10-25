use anyhow::{Context, Ok, bail};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    if args.len() != 2 {
        bail!(
            "cp: expected 2 arguments (<source> <destination>), got {}",
            args.len()
        );
    }
    let src = Path::new(&args[0]);
    let dst_input = Path::new(&args[1]);

    let src_meta = std::fs::symlink_metadata(src)
        .with_context(|| format!("cp: cannot stat '{}'", src.display()))?;

    if src_meta.is_dir() {
        bail!(
            "cp: -r not specified; omitting directory '{}'",
            src.display()
        );
    }
    //1: cp a b         2: cp a dir/
    let dst = resolve_dest_path(dst_input, src)?;

    //copies bytes, overwrites if destination exists, returns copied bytes.
    std::fs::copy(src, &dst)
        .with_context(|| format!("cp: cannot copy '{}' to '{}'", src.display(), dst.display()))?;

    Ok(0)
}

fn resolve_dest_path(dst_input: &Path, src: &Path) -> Result<PathBuf, anyhow::Error> {
    if let std::result::Result::Ok(meta) = std::fs::symlink_metadata(dst_input) {
        if meta.is_dir() {
            //get last component (the file name)
            let name = src.file_name().ok_or_else(|| {
                anyhow::anyhow!(format!("cp: invalid source name '{}'", src.display()))
            })?;
            //appends it and handles separators too
            return Ok(dst_input.join(name));
        }
    }
    Ok(dst_input.to_path_buf())
}
