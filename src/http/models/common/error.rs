use http::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ResponseError {
    #[serde(rename = "id")]
    #[serde(with = "http_serde::option::status_code")]
    pub status: Option<StatusCode>,
    #[serde(rename(deserialize = "$value"))]
    pub text: Option<String>,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest_middleware::Error),
    #[error("HTTP error: {status} - {message}")]
    HttpError { status: StatusCode, message: String },
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_xml_rs::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Unknown error")]
    Unknown,
}
