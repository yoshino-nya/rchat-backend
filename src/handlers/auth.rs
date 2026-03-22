use crate::services::auth::AuthService;
use crate::{AppState, models::user::User};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    user_id: Option<i32>,
    username: Option<String>,
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<User>,
) -> Result<StatusCode, StatusCode> {
    AuthService::register(&state.pool, &req)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<User>,
) -> impl IntoResponse {
    let result = AuthService::login(&state.pool, &req).await;
    match result {
        Ok(id) => (
            StatusCode::OK,
            Json(LoginResponse {
                message: "Login successful.".to_string(),
                user_id: Some(id),
                username: Some(req.username),
            }),
        ),
        Err(e) => {
            tracing::error!("{}", e);
            match e {
                sqlx::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    Json(LoginResponse {
                        message: "账号或密码错误".to_string(),
                        user_id: None,
                        username: None,
                    }),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(LoginResponse {
                        message: "Something went wrong".to_string(),
                        user_id: None,
                        username: None,
                    }),
                ),
            }
        }
    }
}
