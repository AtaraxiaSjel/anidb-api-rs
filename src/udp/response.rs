use std::fmt::Debug;

use super::{ApiError, Result, status::StatusCode};
use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};
use itertools::Itertools;
use tracing::{instrument, trace};

#[derive(Debug)]
pub(crate) struct UdpResponse {
    pub tag: Option<AsciiString>,
    pub status: StatusCode,
    pub message: AsciiString,
    pub data: Vec<AsciiString>,
    pub success: bool,
}

impl UdpResponse {
    /// Parse byte slice into UdpResponse
    #[instrument(skip_all, level = "trace")]
    pub fn from<B>(bytes: &B) -> Result<Self>
    where
        B: AsRef<[u8]> + ?Sized + Debug,
    {
        let data = AsciiStr::from_ascii(bytes)
            .map_err(|err| ApiError::ParseError(err.to_string()))?
            .trim();
        trace!(data = ?data, "parsed bytes into AsciiStr");
        // Split by newlines
        let mut lines = data.split(AsciiChar::LineFeed);
        let header = lines.next().ok_or(ApiError::UnexpectedResponse)?;
        // Parse header
        let mut header = header.trim().split(AsciiChar::Space);
        let first = header.next().ok_or(ApiError::UnexpectedResponse)?;

        let mut tag: Option<AsciiString> = None;
        let status = if let Ok(code) = first.as_str().parse::<u16>() {
            // If first 'word' succefully parsed as integer then it is a status code
            StatusCode::from_u16(code).ok_or(ApiError::UnknownStatusCode(code))?
        } else {
            // Else it's a tag
            tag = Some(first.to_owned());
            let code = header.next().ok_or(ApiError::UnexpectedResponse)?;
            let code = code.as_str().parse::<u16>().map_err(|err| {
                ApiError::ParseError(format!("Failed conversion to u16: {:?}", err.kind()))
            })?;
            StatusCode::from_u16(code).ok_or(ApiError::UnknownStatusCode(code))?
        };
        let ascii_space = [AsciiChar::Space][..].as_ascii_str().unwrap(); // infallible
        let message = Itertools::intersperse(header, ascii_space).collect::<AsciiString>();
        let data = lines
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<AsciiString>>();

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
