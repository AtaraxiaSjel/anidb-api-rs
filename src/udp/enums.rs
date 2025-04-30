use thiserror::Error;

use super::status::ErrorCode;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Timeout error")]
    Timeout,
    #[error("Response error: {0}")]
    ResponseError(ErrorCode),
    #[error("Unexpected response from api")]
    UnexpectedResponse,
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Unknown status code: {0}")]
    UnknownStatusCode(u16),
    #[error("Incorrect username or password")]
    IncorrectUsernameOrPassword,
    #[error("Not logged in")]
    NotLoggedIn,
}
