use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read, Write, BufReader};
use anyhow::Context;

pub fn run(args: &[OsString]) -> Result<i32, anyhow::Error> {
   if args.is_empty() {
    copy_reader_to_stdout(&mut io::stdin().lock())?;
    return Ok(0);
   }

   for arg in args {
      if arg == "-" {
         copy_reader_to_stdout(&mut io::stdin().lock()).context("cat: failed reading from stdin")?;
         continue;
      }

      let path = std::path::Path::new(arg); //convert OsString -> Path
      let file = File::open(path).with_context(|| format!("cat: cannot open '{}'", path.to_string_lossy()))?;
      //to_string_lossy(): prints the path even if it's not valid UTF-8

      let mut reader = BufReader::new(file);
      copy_reader_to_stdout(&mut reader).with_context(|| format!("cat: error reading '{}'", path.to_string_lossy()))?;
   }

   // last newline?
   Ok(0)
}

fn copy_reader_to_stdout<R: Read>(reader: &mut R) -> io::Result<()> {
   let mut stdout = io::stdout().lock();
   io::copy(reader, &mut stdout)?; //takes bytes from reader to stdout (in a loop)
   stdout.flush()?; //ensures everything is written out before returning.
   Ok(())
}