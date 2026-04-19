use std::sync::Arc;
use tokio::sync::mpsc;

// handlers/ws.rs
use crate::{
    AppState,
    models::message::{ChatMessageResponse, CreateChatMessage},
    services::message::MessageService,
};
use axum::{
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WsQuery {
    user_id: i32,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    let user_id = query.user_id;
    if user_id <= 0 {
        return axum::http::StatusCode::BAD_REQUEST.into_response();
    }
    ws.on_upgrade(move |socket| handle_socket(socket, user_id, state))
}

async fn handle_socket(mut socket: WebSocket, my_username: i32, state: Arc<AppState>) {
    let (my_tx, mut my_rx) = mpsc::channel::<ChatMessageResponse>(32);
    {
        let mut clients = state.clients.write().await;
        clients.insert(my_username.clone(), my_tx);
        tracing::info!("用户 {} 上线，当前在线: {}", my_username, clients.len());
    }

    loop {
        tokio::select! {
            Some(Ok(msg)) = socket.recv() => {
                match msg {
                    Message::Text(text) => {
                        match serde_json::from_str::<CreateChatMessage>(&text) {
                            Ok(mut create_msg) => {
                                create_msg.sender_id = my_username;

                                let saved = MessageService::save_message(&state.pool, create_msg.clone(), &state.config.base_url).await;
                                tracing::info!(?saved, ?create_msg);
                                match saved {
                                    Ok(msg) => {
                                        let _ = state.tx.send(msg);
                                    },
                                    Err(e) => {
                                        tracing::error!(%e, ?create_msg,"发送消息失败");
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!(%e, ?text, "错误的消息格式")
                            }
                        }
                    }
                    Message::Close(_) => {
                        break;
                    }
                    _ => {}
                }
            }

            Some(chat_msg) = my_rx.recv() => {
                match serde_json::to_string(&chat_msg) {
                    Ok(json) => {
                        if let Err(e) = socket.send(Message::Text(json.into())).await {
                            eprintln!("Send fail: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("to json fail: {}", e);
                    }
                }
            }
        }
    }

    {
        let mut clients = state.clients.write().await;
        clients.remove(&my_username);
        tracing::info!("用户 {} 下线，当前在线: {}", my_username, clients.len());
    }
}
