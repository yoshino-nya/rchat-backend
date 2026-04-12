use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{
        friend::Status,
        response::ApiResponse,
        session::{ChatSessionType, SessionResponse},
    },
    services::session::find_session_id,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub user_id1: i32,
    pub user_id2: i32,
}
pub async fn get_session_id(
    State(state): State<Arc<AppState>>,
    Json(params): Json<QueryParams>,
) -> impl IntoResponse {
    match find_session_id(&state.pool, params.user_id1, params.user_id2).await {
        Ok(uuid) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(uuid),
            }),
        ),
        Err(e) => {
            tracing::error!(?params, %e, "查找 session_id 失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "查找 session_id 失败".to_string(),
                    data: None::<Uuid>,
                }),
            )
        }
    }
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct SessionListItem {
    pub id: i32,
    pub r#type: ChatSessionType,
    pub uuid: Uuid,
    pub last_msg_id: Option<i32>,
    pub last_msg_sender: Option<String>,
    pub last_msg_time: Option<DateTime<Utc>>,
    pub last_msg_content: Option<String>,
}

pub async fn get_user_sessions(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    // 用户名，创建时间
    let res: Result<Vec<SessionListItem>, _> = sqlx::query_as(
        r#"
        SELECT
            cs.id,
            cs.type,
            cs.uuid,
            cs.last_msg AS last_msg_id,

            m.content AS last_msg_content,
            m.created_time AS last_msg_time,

            u.username AS last_msg_sender

        FROM chat_session_members csm

        JOIN chat_session cs
            ON cs.uuid = csm.session_id

        LEFT JOIN chat_message m
            ON cs.last_msg = m.id

        LEFT JOIN "user" u
            ON u.id = m.sender_id
        
        WHERE csm.user_id = $1
    "#,
    )
    .bind(user_id)
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
            tracing::error!(%e, "获取用户会话失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "ok".to_string(),
                    data: None,
                }),
            )
        }
    }
}
