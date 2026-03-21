use crate::models::friend::{FriendRequest, Status};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    AppState,
    services::friend::{manage_friend_request, save_friend_request},
};

pub async fn create_friend_request(
    State(state): State<AppState>,
    Json(request): Json<FriendRequest>,
) -> impl IntoResponse {
    match save_friend_request(&state.pool, request).await {
        Ok(_) => (StatusCode::OK, "Friend request created"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create friend request",
        ),
    }
}

pub async fn accept_friend_request(
    State(state): State<AppState>,
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
    State(state): State<AppState>,
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
