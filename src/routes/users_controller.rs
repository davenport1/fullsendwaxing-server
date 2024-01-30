use axum::{Extension, Json, http::StatusCode, debug_handler};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, ActiveModelTrait, ColumnTrait};
use serde::{Deserialize, Serialize};

use crate::database::{
    users,
    users::Entity as Users,
};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    user_id: i32,
    username: String,
    token: String,
}

#[debug_handler]
pub async fn create_user(
    Extension(connection): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, StatusCode> {

    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("ashdbfj34t5498tg0sdfopvml".to_owned())),
        ..Default::default()
    }
        .save(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        user_id: new_user.id.unwrap(),
        username: new_user.username.unwrap(),
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
        .filter(users::Column::Username.eq(request_user.username))
        .one(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        // login logic
        let new_token = "ajdsng4359038guivsjld345reaa".to_owned(); // will be changed to random string
        let mut user = db_user.into_active_model();

        // replace with a randomized token
        user.token = Set(Some(new_token));

        let saved_user = user.save(&connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
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
    authorization: TypedHeader<Authorization<Bearer>>
) ->Result<(), StatusCode> {

    let token = authorization.token();

    let mut user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        user.into_active_model()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    user.token = Set(None);

    let _ = user.save(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);


    Ok(())
}

// https://github.com/tokio-rs/axum/discussions/1735