use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    GamePath,
    Mutex,
    DllNotFound,
    ProcNotFound,
    TokenLength,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GamePath => write!(f, "unable to find path to game"),
            Self::Mutex => write!(f, "unable to create game mutexes"),
            Self::DllNotFound => write!(f, "unable to find mhfo-hd.dll in the specified game path"),
            Self::ProcNotFound => write!(f, "unable to find mhDLL_Main proc in mhfo-hd.dll"),
            Self::TokenLength => write!(f, "user token must have a length of 16"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
