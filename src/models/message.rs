// models/message.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::avatar_url;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateChatMessage {
    pub sender_id: i32,
    pub session_id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessage {
    pub id: i32,
    pub sender_id: i32,
    pub session_id: Uuid,
    pub content: String,
    pub created_time: DateTime<Utc>,
    #[sqlx(default)]
    pub sender_avatar: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessageResponse {
    pub id: i32,
    pub sender_id: i32,
    pub session_id: Uuid,
    pub content: String,
    pub created_time: DateTime<Utc>,
    pub sender_avatar: String,
}

impl ChatMessage {
    pub fn to_response(self, base_url: &str) -> ChatMessageResponse {
        ChatMessageResponse {
            id: self.id,
            sender_id: self.sender_id,
            session_id: self.session_id,
            content: self.content,
            created_time: self.created_time,
            sender_avatar: match self.sender_avatar {
                Some(avatar) => avatar_url(base_url, &avatar.to_string()),
                None => avatar_url(
                    base_url,
                    &"8f6ff5fc-b610-4a4f-8a24-f544418a18ee".to_string(),
                ),
            },
        }
    }
}
