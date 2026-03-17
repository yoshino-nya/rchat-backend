use crate::services::auth::AuthService;
use crate::{AppState, models::user::User};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct LoginResponse {
    message: String,
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
        Ok(r) => match r {
            true => (
                StatusCode::OK,
                Json(LoginResponse {
                    message: "Login successful".to_string(),
                }),
            ),
            false => (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse {
                    message: "Invalid username or password".to_string(),
                }),
            ),
        },
        Err(e) => {
            tracing::error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse {
                    message: "Something went wrong".to_string(),
                }),
            )
        }
    }
}
