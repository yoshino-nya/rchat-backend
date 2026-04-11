// services/message.rs
use sqlx::PgPool;

use crate::{ChatMessage, models::message::CreateChatMessage};

pub struct MessageService;

impl MessageService {
    pub async fn get_message_list(pool: &PgPool) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let res: Vec<ChatMessage> = sqlx::query_as(
            "SELECT user_from, user_to, content, created_time from \"chat_message\"",
        )
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn get_user_chat_history(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let res: Vec<ChatMessage> = sqlx::query_as(
            r#"
            SELECT user_from, user_to, content, created_time FROM "chat_message"
            WHERE user_from = $1 OR user_to = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn get_messages2(
        pool: &PgPool,
        user_id1: i32,
        user_id2: i32,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let res: Vec<ChatMessage> = sqlx::query_as(
            r#"
            SELECT user_from, user_to, content, created_time FROM "chat_message"
            WHERE (user_from = $1 AND user_to = $2) OR (user_from = $2 AND user_to = $1)
            "#,
        )
        .bind(user_id1)
        .bind(user_id2)
        .fetch_all(pool)
        .await?;
        Ok(res)
    }

    pub async fn save_message(
        pool: &PgPool,
        chat_msg: CreateChatMessage,
    ) -> Result<bool, sqlx::Error> {
        let ok: String = sqlx::query_scalar(
            r#"
            INSERT INTO "chat_message" (sender_id, session_id, content) VALUES ($1, $2, $3)
            RETURNING created_time
            "#,
        )
        .bind(chat_msg.sender_id)
        .bind(chat_msg.session_id)
        .bind(chat_msg.content)
        .fetch_one(pool)
        .await?;
        tracing::info!("{:?}", ok);
        Ok(true)
    }
}
