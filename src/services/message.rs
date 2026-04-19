// services/message.rs
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::message::{ChatMessage, ChatMessageResponse, CreateChatMessage};

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
        base_url: &str,
    ) -> Result<Vec<ChatMessageResponse>, sqlx::Error> {
        let res: Vec<ChatMessage> = sqlx::query_as(
            r#"
            SELECT
                cm.id,
                cm.content,
                cm.created_time,
                cm.session_id,
                cm.sender_id,
                u.avatar as sender_avatar
            FROM chat_message cm
            JOIN "user" u
                ON u.id = cm.sender_id
            WHERE cm.session_id = $1
        "#,
        )
        .bind(session_id)
        .fetch_all(pool)
        .await?;
        let rsp = res.into_iter().map(|x| x.to_response(base_url)).collect();
        Ok(rsp)
    }

    pub async fn save_message(
        pool: &PgPool,
        chat_msg: CreateChatMessage,
        base_url: &str,
    ) -> Result<ChatMessageResponse, sqlx::Error> {
        let mut msg: ChatMessage = sqlx::query_as(
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
        msg.sender_avatar = sqlx::query_scalar(
            r#"
            SELECT avatar FROM "user"
            WHERE id = $1
        "#,
        )
        .bind(msg.sender_id)
        .fetch_one(pool)
        .await?;
        Ok(msg.to_response(base_url))
    }
}
