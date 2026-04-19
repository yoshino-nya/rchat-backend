use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    RwLock,
    broadcast::{self, Sender},
};

use sqlx::PgPool;
use tokio::sync::mpsc;

use crate::{
    config::AppConfig,
    db::init_db,
    models::message::ChatMessageResponse,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tx: Sender<ChatMessageResponse>,
    pub clients: Arc<RwLock<HashMap<i32, mpsc::Sender<ChatMessageResponse>>>>,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        tx: Sender<ChatMessageResponse>,
        clients: Arc<RwLock<HashMap<i32, mpsc::Sender<ChatMessageResponse>>>>,
        config: AppConfig,
    ) -> Self {
        Self {
            pool,
            tx,
            clients,
            config,
        }
    }
}

pub async fn build_state(config: AppConfig) -> Arc<AppState> {
    let pool = init_db(&config).await;
    let (tx, _) = broadcast::channel(100);
    let clients = Arc::new(RwLock::new(HashMap::new()));
    Arc::new(AppState::new(pool, tx, clients, config))
}
