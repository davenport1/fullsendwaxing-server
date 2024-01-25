
use hyper::StatusCode;

/// <summary>
///  returns nothing () (void) happy path, status code for error path
/// </summary>
pub async fn always_errors() -> Result<(), StatusCode> {
    return Err(StatusCode::IM_A_TEAPOT);
}