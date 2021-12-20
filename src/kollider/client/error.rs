use thiserror::Error;
use crate::kollider::api::error::KolliderError;
use crate::kollider::env::AuthError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwesting server error: {0}")]
    ReqwestErr(#[from] reqwest::Error),
    #[error("Decoding from JSON error: {0}")]
    DecodeErr(#[from] serde_json::Error),
    #[error("Launcher information file error: {0}")]
    ServerErr(#[from] KolliderError),
    #[error("You need authentificate to call {0}")]
    AuthRequired(String),
    #[error("Error while forming authentification heaaders: {0}")]
    AuthError(#[from] AuthError),
}

/// Alias for a `Result` with the error type `self::Error`.
pub type Result<T> = std::result::Result<T, Error>;
