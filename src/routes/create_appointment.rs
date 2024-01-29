use axum::{Extension, Json};
use chrono::{FixedOffset, DateTime};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::Deserialize;

use crate::database::appointments;

#[derive(Deserialize)]
pub struct RequestBody {
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
