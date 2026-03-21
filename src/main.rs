use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use sqlx::PgPool;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    RwLock,
    broadcast::{self, Sender},
    mpsc,
};
use tower_http::cors::{AllowMethods, Any, CorsLayer};

use tracing_subscriber;

use crate::{
    handlers::{
        auth::{login_handler, register_handler},
        friend::{accept_friend_request, create_friend_request, reject_friend_request},
        message::{chat_history_handler, chat_list_handler, chat_messages2_handler},
        user::{get_user_by_id, get_user_by_name},
        ws::ws_handler,
    },
    models::message::ChatMessage,
};
mod handlers;
mod models;
mod services;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    tx: Sender<ChatMessage>,
    clients: Arc<RwLock<HashMap<i32, mpsc::Sender<ChatMessage>>>>,
}
#[tokio::main]
pub async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // 设置日志级别为 INFO
        .init();
    let (tx, _) = broadcast::channel(100);
    let database_url = "postgres://dev:123456@localhost/rchat";
    let pool = PgPool::connect(database_url)
        .await
        .expect("database connect fails!");
    let clients = Arc::new(RwLock::new(HashMap::new()));
    let state = AppState { pool, tx, clients };

    let state_clone = state.clone();
    tokio::spawn(async move {
        let mut rx = state_clone.tx.subscribe();
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    let clients = state_clone.clients.read().await;
                    if let Some(tx) = clients.get(&msg.user_to) {
                        let _ = tx.send(msg).await;
                    }
                }
                Err(_) => break,
            }
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许任何域名访问
        .allow_methods(AllowMethods::any()) // 允许所有 HTTP 方法
        .allow_headers(Any); // 允许任何HTTP头

    let app = Router::new()
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
        .route(
            "/api/friend_request/{id}/accept",
            post(accept_friend_request),
        )
        .route(
            "/api/friend_request/{id}/reject",
            post(reject_friend_request),
        )
        .with_state(state.clone())
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
