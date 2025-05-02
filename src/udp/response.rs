use std::fmt::Debug;

use super::{ApiError, Result, status::StatusCode};
use tracing::{instrument, trace};

#[derive(Debug)]
pub struct UdpResponse {
    pub tag: Option<String>,
    pub status: StatusCode,
    pub message: String,
    pub data: Vec<String>,
    pub success: bool,
}

impl UdpResponse {
    /// Parse byte slice into UdpResponse
    #[instrument(skip_all, level = "trace")]
    pub fn from(bytes: &[u8]) -> Result<Self> {
        let data = String::from_utf8_lossy(bytes);
        trace!(data = ?data, "parsed bytes into String");
        // Split by newlines
        let mut lines = data.lines();
        let header = lines.next().ok_or(ApiError::UnexpectedResponse)?;
        // Parse header
        let mut header = header.split_whitespace();
        let first = header.next().ok_or(ApiError::UnexpectedResponse)?;

        let mut tag: Option<String> = None;
        let status = if let Ok(code) = first.parse::<u16>() {
            // If first 'word' succefully parsed as integer then it is a status code
            StatusCode::from_u16(code).ok_or(ApiError::UnknownStatusCode(code))?
        } else {
            // Else it's a tag
            tag = Some(first.to_owned());
            let code = header.next().ok_or(ApiError::UnexpectedResponse)?;
            let code = code.parse::<u16>().map_err(|err| {
                ApiError::ParseError(format!("Failed conversion to u16: {:?}", err.kind()))
            })?;
            StatusCode::from_u16(code).ok_or(ApiError::UnknownStatusCode(code))?
        };
        let message = itertools::intersperse(header, " ").collect::<String>();
        let data = lines
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<String>>();

        let response = UdpResponse {
            tag,
            status,
            message,
            data,
            success: status.is_success(),
        };

        trace!(response = ?response, "parsed ascii into UdpResponse");

        Ok(response)
    }
}
