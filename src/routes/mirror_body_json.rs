use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn mirror_body_json(Json(body) : Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {message: body.message, message_from_server: "Hello from axum server!".to_owned() })
}