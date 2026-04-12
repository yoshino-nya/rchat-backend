// services/message.rs
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::message::{ChatMessageResponse, CreateChatMessage},
};

pub struct MessageService;

impl MessageService {
    pub async fn get_message_list(pool: &PgPool) -> Result<Vec<ChatMessageResponse>, sqlx::Error> {
        let res: Vec<ChatMessageResponse> = sqlx::query_as(
            "SELECT id, sender_id, session_id, content, created_time from \"chat_message\"",
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn get_session_messages(
        pool: &PgPool,
        session_id: Uuid,
    ) -> Result<Vec<ChatMessageResponse>, sqlx::Error> {
        let res = sqlx::query_as(
            r#"
            SELECT id, sender_id, content, session_id, created_time from "chat_message"
            WHERE session_id = $1
        "#,
        )
        .bind(session_id)
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn save_message(
        pool: &PgPool,
        chat_msg: CreateChatMessage,
    ) -> Result<ChatMessageResponse, sqlx::Error> {
        let msg: ChatMessageResponse = sqlx::query_as(
            r#"
            INSERT INTO "chat_message" (sender_id, session_id, content) VALUES ($1, $2, $3)
            RETURNING id, sender_id, session_id, content, created_time
            "#,
        )
        .bind(chat_msg.sender_id)
        .bind(chat_msg.session_id)
        .bind(chat_msg.content)
        .fetch_one(pool)
        .await?;
        sqlx::query(
            r#"
            UPDATE chat_session
            SET last_msg = GREATEST(last_msg, $1)
            WHERE uuid = $2"#,
        )
        .bind(msg.id)
        .bind(msg.session_id)
        .execute(pool)
        .await?;
        Ok(msg)
    }
}
