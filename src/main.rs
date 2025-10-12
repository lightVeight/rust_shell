use std::io::{self, Write};
use std::ffi::OsString;

mod cmd; // include src/cmd.rs

fn main() -> std::process::ExitCode {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush().ok();

        line.clear();
        if stdin.read_line(&mut line).unwrap_or(0) == 0 {
            break; // EOF (Ctrl+D)
        }

        let word = line.split_whitespace().next();
        match word {
            Some("pwd") => {
                let code = cmd::pwd::run(&[]).unwrap_or_else(|e| {
                    eprintln!("{e}");
                    1
                });
                if code != 0 {
                    return std::process::ExitCode::from(code as u8);
                }
            }
            Some("exit") => break,
            Some(other) => eprintln!("unknown command: {other}"),
            None => {}
        }
    }

    std::process::ExitCode::from(0)
}