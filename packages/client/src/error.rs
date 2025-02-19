use std::fmt::Display;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Error {
    #[error("{0}")]
    Other(String),
    #[error("HTTP request failed with status code {status}")]
    HttpRequestFailure { status: u16 },
}

impl Error {
    pub fn from_other_error<E: Display>(e: E) -> Error {
        Error::Other(e.to_string())
    }
}
