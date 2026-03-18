// models/message.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessage {
    pub user_from: i32,
    pub user_to: i32,
    pub content: String,
    pub created_time: Option<DateTime<Utc>>,
}
