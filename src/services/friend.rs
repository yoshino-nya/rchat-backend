// services/friend.rs
use sqlx::PgPool;

use crate::models::friend::{FriendRequest, Status};

pub async fn save_friend_request(pool: &PgPool, request: FriendRequest) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO friend_request (user_from, user_to) VALUES ($1, $2)
    "#,
    )
    .bind(request.user_from)
    .bind(request.user_to)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn manage_friend_request(
    pool: &PgPool,
    id: i32,
    status: Status,
) -> Result<(), sqlx::Error> {
    match status {
        Status::Accepted => {
            sqlx::query(
                r#"
                UPDATE friend_request
                SET status = 'accepted'
                WHERE id = $1
            "#,
            )
            .bind(id)
            .execute(pool)
            .await?;
        }
        Status::Rejected => {
            sqlx::query(
                r#"
                UPDATE friend_request
                SET status = 'rejected'
                WHERE id = $1
            "#,
            )
            .bind(id)
            .execute(pool)
            .await?;
        }
        _ => {} // status should not to be Status::Pending
    }
    Ok(())
}
