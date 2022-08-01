use crate::kollider::api::error::KolliderError;
use crate::kollider::env::AuthError;
use thiserror::Error;

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
    #[error("Cannot cancel order {0} for ticker {1} due reason: {2}")]
    CancelOrder(u64, String, String),
}

/// Alias for a `Result` with the error type `self::Error`.
pub type Result<T> = std::result::Result<T, Error>;
