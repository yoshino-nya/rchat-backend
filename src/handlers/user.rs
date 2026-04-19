use crate::AppState;
use crate::models::response::ApiResponse;
use crate::models::user::User;
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
    let res: Result<User, _> = sqlx::query_as(
        r#"
        SELECT id, username, avatar FROM "user"
        WHERE id = $1
    "#,
    )
    .bind(user_id)
    .fetch_one(&state.pool)
    .await;
    match res {
        Ok(info) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(info.to_response(&state.config.base_url)),
            }),
        ),
        Err(e) => {
            tracing::error!(%e, ?user_id, "获取用户信息失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "获取用户信息失败".to_string(),
                    data: None,
                }),
            )
        }
    }
}

pub async fn get_user_by_name(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let res: Result<User, _> = sqlx::query_as(
        r#"
        SELECT id, username, avatar FROM "user"
        WHERE username = $1
    "#,
    )
    .bind(&username)
    .fetch_one(&state.pool)
    .await;
    match res {
        Ok(info) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(info.to_response(&state.config.base_url)),
            }),
        ),
        Err(e) => {
            tracing::error!(%e, ?username, "获取用户信息失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "获取用户信息失败".to_string(),
                    data: None,
                }),
            )
        }
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
