use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use chrono::{FixedOffset, DateTime};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::database::appointments;

#[derive(Deserialize)]
pub struct RequestBody {
    user_id: i32,
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


pub async fn create_appointment(Extension(database): Extension<DatabaseConnection>, Json(appointment): Json<RequestBody>) {
    let new_appointment = appointments::ActiveModel {
        // id is set by database
        user_id: Set(appointment.user_id),
        date: Set(appointment.date),
        category: Set(appointment.category),
        notes: Set(appointment.notes),
        ..Default::default()
    };

    let result = new_appointment.save(&database).await.unwrap();
    dbg!(result);
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
