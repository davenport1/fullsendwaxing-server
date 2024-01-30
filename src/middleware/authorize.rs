use axum::http::{Extensions, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::headers::{Authorization, HeaderMapExt};
use axum_extra::headers::authorization::Bearer;
use sea_orm::DatabaseConnection;

pub async fn authorize<T>(
    request: Request<T>,
    next: Next
) -> Result<Response, StatusCode> {
    let token = request.headers().typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();
    let database = request.extensions().get::<DatabaseConnection>()
        .ok_or()
    todo!()
}