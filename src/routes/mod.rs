mod appointments_controller;
mod users_controller;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use hyper::Method;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes(connection: DatabaseConnection) -> Router {
    // set up cors middleware layer
    let cors = CorsLayer::new()
        // Allow the verbs/methods of our choice
        .allow_methods([Method::GET, Method::POST])
        // specify origins of requests
        .allow_origin(Any);

    // build out routes
    Router::new()
        .route("/users", post(users_controller::create_user))
        .route("/users/login", post(users_controller::login))
        .route("/appointments", post(appointments_controller::create_appointment))
        .route("/appointments/:appointment_id", get(appointments_controller::get_appointment))
        .route("/appointments", get(appointments_controller::get_all_appointments))
        .layer(Extension(connection))
        .layer(cors)
}
