use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, broadcast::Sender};

use sqlx::PgPool;
use tokio::sync::mpsc;

use crate::models::message::ChatMessageResponse;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tx: Sender<ChatMessageResponse>,
    pub clients: Arc<RwLock<HashMap<i32, mpsc::Sender<ChatMessageResponse>>>>,
}
