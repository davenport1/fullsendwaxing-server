mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;

use axum::{
    middleware, routing::{get, post}, Extension, Router
};
use hyper::Method;
use tower_http::cors::{Any, CorsLayer};

use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variables::{path_variables, hard_coded_path};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use middleware_message::middleware_message;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use always_errors::always_errors;
use returns_201::returns_201;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {

    // set up cors middleware layer
    let cors = CorsLayer::new()
        // Allow the verbs/methods of our choice
        .allow_methods([Method::GET, Method::POST])
        // specify origins of requests
        .allow_origin(Any);

    let shared_data: SharedData = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    // build out routes
    Router::new()
        .route("/", get(hello_world))
        .route("/post_string", post(mirror_body_string))
        .route("/post_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variabled/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors) // add our cors layer as a middleware function to our routes.. affects all routes above}
        .layer(Extension(shared_data))
        .route("/read_middleware_custom_header", get(read_middleware_custom_header))
        // this route is outside of the middleware layers and will not be able to access the others ^^^
        .route_layer(middleware::from_fn(set_middleware_custom_header)) // will run before all routes above it
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
}