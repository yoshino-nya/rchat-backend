use crate::{
    models::friend::{CreateFriendRequest, DeleteFriendshipRequest, Status},
    services::friend::{delete_friendship, get_friends_service, query_friend_requests},
};
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
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match query_friend_requests(&state.pool, user_id).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_friends_handler(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match get_friends_service(&state.pool, user_id).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
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

pub async fn delete_friendship_handler(
    State(state): State<AppState>,
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
