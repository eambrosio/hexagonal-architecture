use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handler() -> Json<Response> {
    Json(Response {
        message: String::from("ok"),
    })
}
