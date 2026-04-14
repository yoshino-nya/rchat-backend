use std::sync::Arc;

use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{models::response::ApiResponse, services::upload::upload_avatar, state::AppState};

pub async fn update_avatar(
    State(_state): State<Arc<AppState>>,
    multipart: Multipart,
) -> impl IntoResponse {
    tracing::info!("uploading avatar");

    let res = upload_avatar(multipart).await;
    match res {
        Ok(_) => (
            StatusCode::OK,
            Json({
                ApiResponse {
                    message: "ok".to_string(),
                    data: (),
                }
            }),
        ),
        Err(e) => {
            tracing::error!(?e, "上传失败");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "上传失败".to_string(),
                    data: (),
                }),
            )
        }
    }
}
