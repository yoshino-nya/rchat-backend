// models/message.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessage {
    pub sender_id: i32,
    pub session_id: Uuid,
    pub content: String,
    pub created_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateChatMessage {
    pub sender_id: i32,
    pub session_id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatMessageResponse {
    pub id: i32,
    pub sender_id: i32,
    pub session_id: Uuid,
    pub content: String,
    pub created_time: DateTime<Utc>,
}
