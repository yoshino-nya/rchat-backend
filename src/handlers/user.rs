use crate::AppState;
use crate::services::user::UserService;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let res = UserService::find_user_by_id(&state.pool, user_id).await;
    match res {
        Err(e) => {
            // tracing::error!("获取用户信息失败, {}", e);
            match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "用户不存在").into_response(),
                _ => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            }
        }
        Ok(user) => Json(user).into_response(),
    }
}

pub async fn get_user_by_name(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let res = UserService::find_user_by_name(&state.pool, username).await;
    match res {
        Err(e) => {
            // tracing::error!("获取用户信息失败, {}", e);
            match e {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "用户不存在").into_response(),
                _ => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            }
        }
        Ok(user) => Json(user).into_response(),
    }
}
