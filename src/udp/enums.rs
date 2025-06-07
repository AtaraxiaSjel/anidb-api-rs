use thiserror::Error;

use super::status::ErrorCode;

// !TODO: split into public api errors and library ones
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApiError {
    #[error("IO error: {0}")]
    Io(std::io::ErrorKind),
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
    #[error("No UDP Api key specified")]
    NoApiKey,
    #[error("Unknown error")]
    Unknown,
    #[error("Cannot decrypt server response")]
    Decrypt,
    #[error("Banned: {0}")]
    Banned(String),
}
