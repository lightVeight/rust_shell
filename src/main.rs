use std::ffi::OsString;
use std::io::{self, Write};

mod cmd; // include src/cmd.rs

fn main() -> std::process::ExitCode {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().ok();

        line.clear();
        if stdin.read_line(&mut line).unwrap_or(0) == 0 {
            break; // EOF (Ctrl+D)
        }

        let mut parts = line.split_whitespace();
        let cmd_name = match parts.next() {
            Some(name) => name,
            None => continue,
        };
        let args: Vec<OsString> = parts.map(OsString::from).collect();

        match cmd_name {
            "pwd" => {
                let code = cmd::pwd::run(&[]).unwrap_or_else(|e| {
                    eprintln!("{e}");
                    1
                });
                if code != 0 {
                    return std::process::ExitCode::from(code as u8);
                }
            }
            "echo" => {
                if let Err(e) = cmd::echo::run(&args) {
                    eprintln!("{e}");
                }
            }
            "cd" => {
                if let Err(e) = cmd::cd::run(&args) {
                    eprintln!("{e}");
                }
            }
            "cat" => {
                if let Err(e) = cmd::cat::run(&args) {
                    eprintln!("{e}");
                }
            }
            "mkdir" => {
                if let Err(e) = cmd::mkdir::run(&args) {
                    eprintln!("{e}");
                }
            }
            "rmdir" => {
                if let Err(e) = cmd::rmdir::run(&args) {
                    eprintln!("{e}");
                }
            }
            "rm" => {
                if let Err(e) = cmd::rm::run(&args) {
                    eprintln!("{e}");
                }
            }
            "ls" => {
                if let Err(e) = cmd::ls::run(&args) {
                    eprintln!("{e}");
                }
            }
            "exit" => break,
            invalid => eprintln!("Command '{invalid}' not found"),
        }
    }
    std::process::ExitCode::from(0)
}
