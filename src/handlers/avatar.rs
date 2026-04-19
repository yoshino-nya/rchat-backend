use std::sync::Arc;

use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{models::response::ApiResponse, services::upload::upload_avatar, state::AppState};

pub async fn update_avatar(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    multipart: Multipart,
) -> impl IntoResponse {
    let res = upload_avatar(&state.pool, multipart, user_id).await;
    match res {
        Ok(uuid) => {
            let s = uuid.to_string();
            (
                StatusCode::OK,
                Json({
                    ApiResponse {
                        message: "ok".to_string(),
                        data: Some(format!(
                            "{}/uploads/avatars/{}/{}.jpg",
                            state.config.base_url,
                            &s[0..2],
                            s
                        )),
                    }
                }),
            )
        }
        Err(e) => {
            tracing::error!(?e, "上传失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "上传失败".to_string(),
                    data: None,
                }),
            )
        }
    }
}
