use axum::http::StatusCode;
use axum::{response::Response, middleware::Next, extract::Request};
use axum_extra::headers::{Authorization, HeaderMapExt};
use axum_extra::headers::authorization::Bearer;
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter, EntityTrait};

use crate::database::users;
use crate::database::users::Entity as Users;

pub async fn authorize(
    mut request: Request,
    next: Next
) -> Result<Response, StatusCode> {

    let token = request.headers().typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // if there is no user with the token, the token is invalid or expired
    let Some(user) = user else { return Err(StatusCode::UNAUTHORIZED) };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}