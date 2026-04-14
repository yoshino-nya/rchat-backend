use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, broadcast};

use crate::{
    db::init_db, router::build_router, state::AppState,
    tasks::spawn_message_dispatcher,
};
mod db;
mod handlers;
mod models;
mod router;
mod services;
mod state;
mod tasks;
mod tracing;

#[tokio::main]
pub async fn main() {
    tracing::init();

    let (tx, _) = broadcast::channel(100);
    let pool = init_db().await;
    let clients = Arc::new(RwLock::new(HashMap::new()));
    let state = Arc::new(AppState { pool, tx, clients });

    spawn_message_dispatcher(state.clone());

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
