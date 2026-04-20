use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            message: "ok".to_string(),
            data: Some(data),
        }
    }

    pub fn err(msg: String) -> Self {
        Self {
            message: msg,
            data: None,
        }
    }
}

impl<T: serde::Serialize> axum::response::IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
