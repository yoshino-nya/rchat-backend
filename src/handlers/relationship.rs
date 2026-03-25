use crate::{AppState, services::relationship::get_relationship_service};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn get_relationship_handler(
    State(state): State<AppState>,
    Path((mut user_id_1, mut user_id_2)): Path<(i32, i32)>,
) -> impl IntoResponse {
    if user_id_1 > user_id_2 {
        std::mem::swap(&mut user_id_1, &mut user_id_2);
    }
    match get_relationship_service(&state.pool, user_id_1, user_id_2).await {
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
    }
}
