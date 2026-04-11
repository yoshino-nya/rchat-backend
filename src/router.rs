use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};
use tower_http::cors::{AllowMethods, Any, CorsLayer};

use crate::{
    handlers::{
        auth::*, friend::*, message::*, relationship::*, session::get_session_id, user::*, ws::*,
    },
    state::AppState,
};

pub fn build_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(AllowMethods::any())
        .allow_headers(Any);
    Router::new()
        .route("/", axum::routing::get(|| async { "Hello World" }))
        .route("/api/register", post(register_handler))
        .route("/api/login", post(login_handler))
        .route("/ws", get(ws_handler))
        .route("/api/messages", get(chat_list_handler))
        .route("/api/users/id/{id}", get(get_user_by_id))
        .route("/api/users/name/{name}", get(get_user_by_name))
        .route("/api/users/{id}/messages", get(chat_history_handler))
        .route("/api/messages/{id1}/{id2}", get(chat_messages2_handler))
        .route("/api/friend_request", post(create_friend_request))
        .route("/api/users/{id}/friend_requests", get(get_friend_requests))
        .route(
            "/api/friend_request/{id}/accept",
            post(accept_friend_request),
        )
        .route(
            "/api/friend_request/{id}/reject",
            post(reject_friend_request),
        )
        .route("/api/friend_request", delete(delete_friendship_handler))
        .route("/api/users/{id}/friends", get(get_friends_handler))
        .route(
            "/api/relationships/{id1}/{id2}",
            get(get_relationship_handler),
        )
        .route("/api/users/search", get(search_user_by_name))
        .route("/api/sessions/private", post(get_session_id))
        .with_state(state)
        .layer(cors)
}
