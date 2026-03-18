use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{AppState, services::message::MessageService};

// GET /api/meesages
pub async fn chat_list_handler(State(state): State<AppState>) -> impl IntoResponse {
    let res = MessageService::get_message_list(&state.pool).await;
    match res {
        Ok(list) => Json(list).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// GET /api/users/{id}/messages
pub async fn chat_history_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let res = MessageService::get_user_chat_history(&state.pool, id).await;
    match res {
        Ok(list) => Json(list).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// GET /api/messages/{id1}/{id2}
pub async fn chat_messages2_handler(
    State(state): State<AppState>,
    Path((usre_id1, user_id2)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let res = MessageService::get_messages2(&state.pool, usre_id1, user_id2).await;
    match res {
        Ok(list) => Json(list).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
