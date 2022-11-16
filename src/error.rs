use crate::Level;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The requested '{0}' with does not exist")]
    NotFound(&'static str, String),

    #[error("Request failed {0}: {1}")]
    RemoteFailed(u16, String),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    Kv(#[from] worker::kv::KvError),

    #[error(transparent)]
    Worker(#[from] worker::Error),

    #[error("{0}")]
    BadRequest(String),

    #[error("Access Denied")]
    AccessDenied,

    #[error("{0}")]
    // error, XML
    InvalidPoB(String, String),

    #[error("{0}")]
    InvalidId(&'static str),

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    Error(String),
}

impl Error {
    pub fn level(&self) -> Level {
        match self {
            Self::NotFound(..) => Level::Info,
            Self::RemoteFailed(..) => Level::Warning,
            Self::Serde(..) => Level::Error,
            Self::Kv(..) => Level::Error,
            Self::Worker(..) => Level::Error,
            Self::BadRequest(..) => Level::Info,
            Self::AccessDenied => Level::Info,
            Self::InvalidId(..) => Level::Info,
            Self::InvalidPoB(..) => Level::Error,
            Self::IOError(..) => Level::Error,
            Self::Error(..) => Level::Error,
        }
    }
}
