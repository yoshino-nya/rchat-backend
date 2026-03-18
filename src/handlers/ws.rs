use tokio::sync::mpsc;

// handlers/ws.rs
use crate::{AppState, models::message::ChatMessage, services::message::MessageService};
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
    user_from: i32,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    let user_from = query.user_from;
    if user_from <= 0 {
        return axum::http::StatusCode::BAD_REQUEST.into_response();
    }
    ws.on_upgrade(move |socket| handle_socket(socket, user_from, state))
}

async fn handle_socket(mut socket: WebSocket, my_username: i32, state: AppState) {
    let (my_tx, mut my_rx) = mpsc::channel::<ChatMessage>(32);
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
                        match serde_json::from_str::<ChatMessage>(&text) {
                            Ok(mut chat_msg) => {
                                chat_msg.user_from = my_username.clone();

                                let _ = MessageService::save_message(&state.pool, chat_msg.clone()).await;

                                if let Err(e) = state.tx.send(chat_msg) {
                                    eprintln!("Send fail: {}", e);
                                }

                            }
                            Err(e) => {
                                eprint!("Invalid message: {}", e);
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
