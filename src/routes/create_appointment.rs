use axum::Extension;
use sea_orm::DatabaseConnection;

use crate::database::appointments;

pub async fn create_appointment(Extension(database): Extension<DatabaseConnection>) {
    let new_appointment = appointments::ActiveModel {
        id: Default::default(),
        user_id: Default::default(),
        date: Default::default(),
        start_time: Default::default(),
        end_time: Default::default(),
        category: Default::default(),
        notes: Default::default(),
    };
}