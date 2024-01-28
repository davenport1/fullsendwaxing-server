mod create_appointment;
mod get_appointment;

use axum::{
    routing::{get, post}, Extension, Router
};
use hyper::Method;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use create_appointment::create_appointment;
use get_appointment::get_appointment;


pub fn create_routes(connection: DatabaseConnection) -> Router {

    // set up cors middleware layer
    let cors = CorsLayer::new()
        // Allow the verbs/methods of our choice
        .allow_methods([Method::GET, Method::POST])
        // specify origins of requests
        .allow_origin(Any);

    // build out routes
    Router::new()
        .route("/appointments", post(create_appointment))
        .route("/appointments", get(get_appointment))
        .layer(Extension(connection))
        .layer(cors)
}