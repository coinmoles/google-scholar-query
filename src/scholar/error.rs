use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Parse error")]
    ParseError,
    #[error("Invalid URL")]
    InvalidServiceError,
    #[error("Invalid URL")]
    RequiredFieldError,
    #[error("Not implemented")]
    NotImplementedError,
    #[error("Invalid Response")]
    InvalidResponseError,
}
