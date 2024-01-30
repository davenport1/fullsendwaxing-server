use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use chrono::{FixedOffset, DateTime};
use sea_orm::{ActiveValue::Set, entity::*, QueryFilter};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::database::{appointments, users};
use crate::database::users::Entity as Users;

#[derive(Deserialize)]
pub struct RequestBody {
    date: DateTime<FixedOffset>,
    category: Option<i32>,
    notes: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseBody {
    id: i32,
    user_id: i32,
    date: DateTime<FixedOffset>,
    category: Option<i32>,
    notes: Option<String>,
}

pub async fn create_appointment(
    Extension(database): Extension<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(appointment): Json<RequestBody>
) -> Result<Json<ResponseBody>, StatusCode> {

    let token = authorization.token();

    // authorize user
    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_appointment = appointments::ActiveModel {
        // id is set by database
        user_id: Set(user.id),
        date: Set(appointment.date),
        category: Set(appointment.category),
        notes: Set(appointment.notes),
        ..Default::default()
    };

    let result = new_appointment.save(&database).await.unwrap();

    Ok(Json(ResponseBody {
        id: result.id.unwrap(),
        user_id: result.user_id.unwrap(),
        date: result.date.unwrap(),
        category: result.category.unwrap(),
        notes: result.notes.unwrap(),
    }))
}

pub async fn get_appointment(
    Extension(connection): Extension<DatabaseConnection>,
    Path(appointment_id): Path<i32>) -> Result<Json<ResponseBody>, StatusCode>{

    let appointment = appointments::Entity::find_by_id(appointment_id)
        .one(&connection)
        .await
        .unwrap();

    if let Some(appointment) = appointment {
        Ok(Json(
            ResponseBody {
                id: appointment_id,
                user_id: appointment.user_id,
                date: appointment.date,
                category: appointment.category,
                notes: appointment.notes,
            }
        ))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_appointments(
    Extension(connection): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseBody>>, StatusCode> {
    let appointments = appointments::Entity::find()
        .all(&connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;



    todo!()
}
