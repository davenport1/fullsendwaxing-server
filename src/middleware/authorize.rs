use axum::http::StatusCode;
use axum::{response::Response, middleware::Next, extract::Request};
use axum_extra::headers::{Authorization, HeaderMapExt};
use axum_extra::headers::authorization::Bearer;
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter, EntityTrait};

use crate::database::users;
use crate::database::users::Entity as Users;
use crate::utilities::claims::is_token_valid;
use crate::utilities::fsw_error::FswError;

pub async fn authorize(
    mut request: Request,
    next: Next
) -> Result<Response, FswError> {
    let token = request.headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| FswError::new(StatusCode::BAD_REQUEST, "Missing bearer token"))?
        .token()
        .to_owned();

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| FswError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong, please try again later."))?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|_| FswError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong, please try again later."))?;

    // done after to obfuscate that the token is wrong
    is_token_valid(&token)?; // verify valid claim using claims utility after getting from database
    // we could update this token everytime it gets used and refresh the time its valid.

    // if there is no user with the token, the token is invalid or expired
    let Some(user) = user
        else { return Err(FswError::new(StatusCode::UNAUTHORIZED,
                                        "You are not authorized, please login or create a new account")) };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}

pub async fn authorize_admin(
    mut request: Request,
    next: Next
) -> Result<Response, StatusCode> {
    todo!()
}