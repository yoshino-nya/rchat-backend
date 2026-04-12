use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "chat_session_type", rename_all = "lowercase")]
pub enum ChatSessionType {
    Private,
    Group,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct SessionResponse {
    id: i32,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    session_type: ChatSessionType,
    last_msg: i32,
    uuid: Uuid,
}
