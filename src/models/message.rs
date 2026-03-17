// models/message.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
