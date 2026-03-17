use tokio::sync::mpsc;

// handlers/ws.rs
use crate::{
    AppState,
    models::{message::ChatMessage, user},
};
use axum::{
    Json,
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::query;
use tokio::sync::broadcast;

#[derive(Deserialize)]
struct WsQuery {
    user_from: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    let user_from = query.user_from.trim().to_string();
    if user_from.is_empty() {
        return axum::http::StatusCode::BAD_REQUEST.into_response();
    }
    ws.on_upgrade(move |socket| handle_socket(socket, user_from, state))
}

async fn handle_socket(mut socket: WebSocket, my_username: String, state: AppState) {
    let (my_tx, mut my_rx) = mpsc::channel::<ChatMessage>(32);
    {
        let mut clients = state.clients.write().await;
        clients.insert(my_username.clone(), my_tx);
    }
}
