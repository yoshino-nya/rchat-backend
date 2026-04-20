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
    handlers::avatar,
    models::{
        response::ApiResponse,
        session::{ChatSessionType, SessionResponse},
    },
    services::session::{create_session, find_session_id},
    state::AppState,
    utils::{avatar_url, session_avatar_from_uuid},
};

pub async fn get_chat_session(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let res: Result<SessionResponse, _> = sqlx::query_as(
        r#"
        SELECT id, type, name, last_msg, uuid FROM chat_session
        WHERE uuid = $1
    "#,
    )
    .bind(uuid)
    .fetch_one(&state.pool)
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
            tracing::error!(%e, "获取会话详情失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "获取会话详情失败".to_string(),
                    data: None,
                }),
            )
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupParmas {
    pub users: Vec<i32>,
}

pub async fn create_group_session(
    State(state): State<Arc<AppState>>,
    Json(params): Json<CreateGroupParmas>,
) -> impl IntoResponse {
    let res = create_session(&state.pool, params.users, ChatSessionType::Group).await;
    match res {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: Some(res),
            }),
        ),
        Err(e) => {
            tracing::error!(%e, "创建群聊失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "创建群聊失败".to_string(),
                    data: None,
                }),
            )
        }
    }
}

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
    pub name: String,
    pub last_msg_id: Option<i32>,
    pub last_msg_sender: Option<String>,
    pub last_msg_time: Option<DateTime<Utc>>,
    pub last_msg_content: Option<String>,
    pub avatar: Option<Uuid>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct SessionListItemResponse {
    pub id: i32,
    pub r#type: ChatSessionType,
    pub uuid: Uuid,
    pub name: String,
    pub last_msg_id: Option<i32>,
    pub last_msg_sender: Option<String>,
    pub last_msg_time: Option<DateTime<Utc>>,
    pub last_msg_content: Option<String>,
    pub avatar: String,
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

            CASE
                WHEN cs.type = 'group' THEN cs.name
                ELSE COALESCE(csm.remark, u_other.username)
            END AS name,

            CASE 
                WHEN cs.type = 'group' THEN cs.avatar
                ELSE u_other.avatar
            END AS avatar,

            m.content AS last_msg_content,
            m.created_time AS last_msg_time,
            u_sender.username AS last_msg_sender

        FROM chat_session_members csm
        JOIN chat_session cs ON cs.uuid = csm.session_id
        LEFT JOIN chat_message m ON cs.last_msg = m.id
        LEFT JOIN "user" u_sender ON u_sender.id = m.sender_id
        LEFT JOIN chat_session_members csm_other
            ON csm_other.session_id = cs.uuid
            AND csm_other.user_id != $1
            AND cs.type = 'private'
        LEFT JOIN "user" u_other ON u_other.id = csm_other.user_id
        WHERE csm.user_id = $1
    "#,
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await;
    match res {
        Ok(res) => {
            let res: Vec<SessionListItemResponse> = res
                .into_iter()
                .map(|x| SessionListItemResponse {
                    id: x.id,
                    r#type: x.r#type.clone(),
                    uuid: x.uuid,
                    name: x.name,
                    last_msg_id: x.last_msg_id,
                    last_msg_sender: x.last_msg_sender,
                    last_msg_time: x.last_msg_time,
                    last_msg_content: x.last_msg_content,
                    avatar: session_avatar_from_uuid(&state.config.base_url, x.avatar, x.r#type),
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

#[derive(Debug, Deserialize)]
pub struct PatchParams {
    name: Option<String>,
}

pub async fn update_session(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<Uuid>,
    Json(params): Json<PatchParams>,
) -> impl IntoResponse {
    tracing::info!(?params, ?session_id, "更新 session");
    let res = sqlx::query(
        r#"
        UPDATE chat_session
        SET name = COALESCE($1, name)
        WHERE uuid = $2
    "#,
    )
    .bind(params.name)
    .bind(session_id)
    .execute(&state.pool)
    .await;
    match res {
        Ok(_res) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "ok".to_string(),
                data: None::<()>,
            }),
        ),
        Err(e) => {
            tracing::error!(%e, "更新 session 失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "更新 session 失败".to_string(),
                    data: None,
                }),
            )
        }
    }
}
