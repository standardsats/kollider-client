use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse URL of Websocket: {0}")]
    UrlDecode(#[from] url::ParseError),
    #[error("Websocket operation error: {0}")]
    SocketError(#[from] tungstenite::error::Error),
}

/// Alias for a `Result` with the error type `self::Error`.
pub type Result<T> = std::result::Result<T, Error>;
