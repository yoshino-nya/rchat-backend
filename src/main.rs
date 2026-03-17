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

use tracing_subscriber;

use crate::{
    handlers::{
        auth::{login_handler, register_handler},
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
    clients: Arc<RwLock<HashMap<String, mpsc::Sender<ChatMessage>>>>,
}
#[tokio::main]
pub async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // 设置日志级别为 INFO
        .init();
    let (tx, mut rx1) = broadcast::channel(100);
    let database_url = "postgres://dev:123456@localhost/rchat";
    let pool = PgPool::connect(database_url)
        .await
        .expect("database connect fails!");
    let clients = Arc::new(RwLock::new(HashMap::new()));
    let state = AppState { pool, tx, clients };
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello World" }))
        .route("/api/register", post(register_handler))
        .route("/api/login", post(login_handler))
        .route("/ws", get(ws_handler))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
