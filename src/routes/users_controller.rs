use axum::{Extension, Json, http::StatusCode, debug_handler};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, ActiveModelTrait, ColumnTrait};
use serde::{Deserialize, Serialize};

use crate::database::{
    users,
    users::Entity as Users,
};
use crate::database::users::Model as UsersModel;
use crate::utilities::claims;

#[derive(Deserialize)]
pub struct RequestUser {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    user_id: i32,
    email: String,
    token: String,
}

#[debug_handler]
pub async fn create_user(
    Extension(connection): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt = claims::create_token()?;

    let new_user = users::ActiveModel {
        email: Set(request_user.email),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(jwt.to_owned())),
        ..Default::default()
    }
        .save(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        user_id: new_user.id.unwrap(),
        email: new_user.email.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

/// <summary>
///
/// </summary>
#[debug_handler]
pub async fn login(
    Extension(connection): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, StatusCode> {

    let db_user = Users::find()
        .filter(users::Column::Email.eq(request_user.email))
        .one(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }

        // login logic
        let new_token = claims::create_token()?.to_owned(); // will be changed to random string
        let mut user = db_user.into_active_model();

        // replace with a randomized token
        user.token = Set(Some(new_token));

        let saved_user = user.save(&connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            email: saved_user.email.unwrap(),
            user_id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        // cannot find user
        Err(StatusCode::NOT_FOUND)
    }
}

#[debug_handler]
pub async fn logout(
    Extension(connection): Extension<DatabaseConnection>,
    Extension(user): Extension<UsersModel>,
) ->Result<(), StatusCode> {
    // set the token to none and update the users token in db
    let mut user = user.into_active_model();
    user.token = Set(None);

    let _ = user.save(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);


    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
// https://github.com/tokio-rs/axum/discussions/1735