use std::io;
pub type ExitCode = i32;

pub trait Builtin {
    fn run(&self, args: &[std::ffi::OsString]) -> Result<ExitCode, anyhow::Error>;
}

#[derive(Debug, Clone, Copy)]
pub enum BuiltinKind {
    Cat, Cd, Cp, Echo, Exit, Ls, Mkdir, Mv, Pwd, Rm,
}

impl BuiltinKind {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "cat" => Some(Self::Cat),
            "cd" => Some(Self::Cd),
            "cp" => Some(Self::Cp),
            "echo" => Some(Self::Echo),
            "exit" => Some(Self::Exit),
            "ls" => Some(Self::Ls),
            "mkdir" => Some(Self::Mkdir),
            "mv" => Some(Self::Mv),
            "pwd" => Some(Self::Pwd),
            "rm" => Some(Self::Rm),
            _ => None,
        }
    }
}
