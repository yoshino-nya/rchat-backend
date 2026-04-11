use crate::AppState;
use crate::services::user::UserService;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
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
    State(state): State<Arc<AppState>>,
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

#[derive(Debug, serde::Deserialize)]
pub struct SerchParams {
    username: String,
}
pub async fn search_user_by_name(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SerchParams>,
) -> impl IntoResponse {
    tracing::info!("{:?}", params);
    match UserService::query_users_by_name(&state.pool, params.username).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
