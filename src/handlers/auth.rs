use crate::AppState;
use crate::models::response::ApiResponse;
use crate::models::user::{LoginRequest, RegisterRequest};
use crate::services::auth::AuthService;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use std::sync::Arc;

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    AuthService::register(&state.pool, &req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = AuthService::login(&state.pool, &req, &state.config.jwt_secret).await;
    match result {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(res),
            }),
        ),
        Err(e) => {
            let msg = "登录失败".to_string();
            tracing::error!(%e, ?msg);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: msg,
                    data: None,
                }),
            )
        }
    }
}
