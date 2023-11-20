use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    GamePath,
    Mutex,
    GlobalAlloc,
    Dll,
    ProcNotFound,
    TokenLength,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GamePath => write!(f, "unable to find path to game"),
            Self::Mutex => write!(f, "unable to create or free game mutexes"),
            Self::GlobalAlloc => write!(f, "unable to create or free game global alloc"),
            Self::Dll => write!(f, "unable to load or free mhfo dll"),
            Self::ProcNotFound => write!(f, "unable to find mhDLL_Main proc in mhfo-hd.dll"),
            Self::TokenLength => write!(f, "user token must have a length of 16"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
