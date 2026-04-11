use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::response::ApiResponse, services::session::find_session_id, state::AppState};

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
