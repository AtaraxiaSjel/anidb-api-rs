use super::{
    ApiError, Result,
    status::{ErrorCode, StatusCode},
};

pub(crate) fn anidb_encode(input: &str) -> String {
    input.replace('&', "&amp;").replace('\n', "<br />")
}

pub(crate) fn anidb_decode(input: &str) -> String {
    input
        .replace("<br />", "\n")
        .replace('`', "'")
        .replace('/', "|")
}

// !TODO: rewrite as macros with multiple status codes to check
pub(crate) fn check_status(want: StatusCode, got: StatusCode) -> Result<()> {
    if want != got {
        return if want.is_error() {
            Err(ApiError::ResponseError(
                ErrorCode::try_from(want).unwrap(), // infallible
            ))
        } else {
            Ok(())
        };
    }
    Ok(())
}
