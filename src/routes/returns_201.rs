use axum::{response::Response, response::IntoResponse};
use hyper::StatusCode;

pub async fn returns_201() -> Response {
    (
        StatusCode::CREATED,
        "This is a 201 resource created",
    ).into_response()
}