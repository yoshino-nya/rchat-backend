use crate::{
    AppState,
    models::{message::ChatMessageResponse, response::ApiResponse},
    services::message::MessageService,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

// GET /api/meesages
pub async fn chat_list_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let res = MessageService::get_message_list(&state.pool).await;
    match res {
        Ok(list) => Json(list).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn session_messages_handler(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
) -> impl IntoResponse {
    match MessageService::get_session_messages(&state.pool, session_id).await {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(res),
            }),
        ),
        Err(e) => {
            tracing::error!(%e, ?session_id, "获取聊天记录失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "获取聊天记录失败".to_string(),
                    data: None::<Vec<ChatMessageResponse>>,
                }),
            )
        }
    }
}

pub async fn get_message(
    State(state): State<Arc<AppState>>,
    Path(message_id): Path<i32>,
) -> impl IntoResponse {
    let res: Result<Vec<ChatMessageResponse>, _> = sqlx::query_as(
        r#"
        SELECT id, sender_id, session_id, content, created_time
        FROM chat_message WHERE id = $1
    "#,
    )
    .bind(message_id)
    .fetch_all(&state.pool)
    .await;
    match res {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(res),
            }),
        ),
        Err(e) => {
            tracing::error!(%e, "获取消息失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "获取消息失败".to_string(),
                    data: None,
                }),
            )
        }
    }
}
