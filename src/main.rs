use crate::{
    config::AppConfig,
    router::build_router,
    state::{AppState, build_state},
    tasks::spawn_message_dispatcher,
};
mod config;
mod db;
mod handlers;
mod models;
mod router;
mod services;
mod state;
mod tasks;
mod tracing;
mod utils;

#[tokio::main]
pub async fn main() {
    tracing::init();

    dotenv::dotenv().ok();

    let config = AppConfig::load_from_env();
    let state = build_state(config.clone()).await;

    spawn_message_dispatcher(state.clone());

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind(&config.bind_addr())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
