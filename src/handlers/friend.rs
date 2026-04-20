use crate::{
    models::{
        friend::{CreateFriendRequest, DeleteFriendshipRequest, Status},
        response::ApiResponse,
    },
    services::friend::{delete_friendship, get_friends_service, query_friend_requests},
    utils::avatar_url_from_uuid,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    services::friend::{manage_friend_request, save_friend_request},
};

pub async fn create_friend_request(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateFriendRequest>,
) -> impl IntoResponse {
    match save_friend_request(&state.pool, request).await {
        Ok(_) => (StatusCode::OK, "Friend request created"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create friend request",
        ),
    }
}

pub async fn get_friend_requests(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match query_friend_requests(&state.pool, user_id, &state.config.base_url).await {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(res),
            }),
        ),
        Err(e) => {
            let msg = "获取好友请求失败".to_string();
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

pub async fn get_friends_handler(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match get_friends_service(&state.pool, user_id).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn accept_friend_request(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match manage_friend_request(&state.pool, id, Status::Accepted).await {
        Ok(_) => (StatusCode::OK, "Friend request accepted"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to accept friend request",
        ),
    }
}

pub async fn reject_friend_request(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match manage_friend_request(&state.pool, id, Status::Rejected).await {
        Ok(_) => (StatusCode::OK, "Friend request rejected"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to reject friend request",
        ),
    }
}

pub async fn delete_friendship_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DeleteFriendshipRequest>,
) -> impl IntoResponse {
    let (mut user_a, mut user_b) = (req.user_a, req.user_b);
    if user_a > user_b {
        std::mem::swap(&mut user_a, &mut user_b);
    }
    match delete_friendship(&state.pool, user_a, user_b).await {
        Ok(true) => (
            StatusCode::OK,
            Json("Friendship deleted successfully".to_string()),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json("Friendship not found".to_string()),
        ),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct FriendsInfo {
    pub friend_id: i32,
    pub friend_name: String,
    pub created_time: DateTime<Utc>,
    pub friend_avatar: Option<Uuid>,
}
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct FriendsInfoResponse {
    pub friend_id: i32,
    pub friend_name: String,
    pub created_time: DateTime<Utc>,
    pub friend_avatar: String,
}

pub async fn get_friends_details(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let res: Result<Vec<FriendsInfo>, _> = sqlx::query_as(
        r#"
        SELECT
            u.id as friend_id,
            u.avatar as friend_avatar,
            u.username as friend_name,
            fs.created_time
        FROM friendship fs
        JOIN "user" u ON u.id = CASE
            WHEN fs.user_low = $1 THEN fs.user_high
            ELSE fs.user_low
        END
        WHERE fs.user_low = $1 OR fs.user_high = $1
    "#,
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await;
    match res {
        Ok(res) => {
            let res: Vec<FriendsInfoResponse> = res
                .into_iter()
                .map(|f| FriendsInfoResponse {
                    friend_id: f.friend_id,
                    friend_name: f.friend_name,
                    created_time: f.created_time,
                    friend_avatar: avatar_url_from_uuid(&state.config.base_url, f.friend_avatar),
                })
                .collect();
            (
                StatusCode::OK,
                Json(ApiResponse {
                    message: "ok".to_string(),
                    data: Some(res),
                }),
            )
        }
        Err(e) => {
            tracing::error!(%e, "获取好友信息失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "获取好友信息失败".to_string(),
                    data: None,
                }),
            )
        }
    }
}
