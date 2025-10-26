use anyhow::Context;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
    let mut operands: Vec<OsString> = Vec::new();
    let mut stop_flags = false;

    if args.is_empty() {
        copy_reader_to_stdout(&mut io::stdin().lock())?;
        return Ok(0);
    }

    for a in args {
        if !stop_flags {
            if a == "--" {
                stop_flags = true;
                continue;
            }
        }
        operands.push(a.clone());
    }

    if operands.is_empty() {
        copy_reader_to_stdout(&mut io::stdin().lock())?;
        return Ok(0);
    }

    for arg in operands {
        if arg == "-" {
            copy_reader_to_stdout(&mut io::stdin().lock())
                .context("cat: failed reading from stdin")?;
            continue;
        }

        let path = std::path::Path::new(&arg);
        let file = File::open(path)
            .with_context(|| format!("cat: cannot open '{}'", path.to_string_lossy()))?;

        let mut reader = BufReader::new(file);
        copy_reader_to_stdout(&mut reader)
            .with_context(|| format!("cat: error reading '{}'", path.to_string_lossy()))?;
    }

    Ok(0)
}

fn copy_reader_to_stdout<R: Read>(reader: &mut R) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    io::copy(reader, &mut stdout)?;
    stdout.flush()?;
    Ok(())
}
