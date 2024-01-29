use axum::{extract::Path, Extension, Json};
use chrono::{DateTime, FixedOffset};
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::appointments::Entity as Appointments;



#[derive(Serialize)]
pub struct ResponseBody {
    id: i32,
    user_id: i32,
    date: DateTime<FixedOffset>,
    category: Option<i32>,
    notes: Option<String>,
}

pub async fn get_appointment(
    Extension(connection): Extension<DatabaseConnection>,
    Path(appointment_id): Path<i32>) -> Result<Json<ResponseBody>, StatusCode>{

    let appointment = Appointments::find_by_id(appointment_id)
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
