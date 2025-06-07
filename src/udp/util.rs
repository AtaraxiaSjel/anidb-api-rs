use super::{
    ApiError,
    status::{ErrorCode, StatusCode},
};

pub(crate) fn check_status(status: StatusCode) -> ApiError {
    let error_code = ErrorCode::try_from(status);
    if let Ok(err) = error_code {
        ApiError::ResponseError(err)
    } else {
        ApiError::UnexpectedResponse
    }
}
