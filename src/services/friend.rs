// services/friend.rs
use sqlx::{PgPool, Row};

use crate::{
    models::friend::{CreateFriendRequest, FriendRequest, FriendRequestResponse, Status},
    utils::avatar_url_from_uuid,
};

pub async fn save_friend_request(
    pool: &PgPool,
    request: CreateFriendRequest,
) -> Result<(), sqlx::Error> {
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

pub async fn get_friends_service(pool: &PgPool, user_id: i32) -> Result<Vec<i32>, sqlx::Error> {
    let res = sqlx::query(
        r#"
        SELECT
            CASE
                WHEN user_low = $1 THEN user_high
                ELSE user_low
            END AS friend_id
        FROM friendship
        WHERE user_low = $1 OR user_high = $1
    "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| row.get::<i32, _>("friend_id"))
    .collect();
    Ok(res)
}

pub async fn query_friend_requests(
    pool: &PgPool,
    user_id: i32,
    base_url: &str,
) -> Result<Vec<FriendRequestResponse>, sqlx::Error> {
    let res: Vec<FriendRequest> = sqlx::query_as(
        r#"
        SELECT
            fr.id,
            fr.user_from,
            fr.user_to,
            fr.status,
            fr.created_time,
            u.username as user_name,
            u.avatar as user_avatar
        FROM friend_request fr
        JOIN "user" u ON u.id = (
            CASE
                WHEN fr.user_from = $1 THEN fr.user_to
                ELSE fr.user_from
            END
        )
        WHERE fr.user_from = $1 OR fr.user_to = $1
        ORDER BY fr.created_time DESC
    "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    let res = res
        .into_iter()
        .map(|x| FriendRequestResponse {
            user_from: x.user_from,
            user_to: x.user_to,
            status: x.status,
            created_time: x.created_time,
            id: x.id,
            user_name: x.user_name,
            user_avatar: avatar_url_from_uuid(base_url, x.user_avatar),
        })
        .collect();
    Ok(res)
}

pub async fn delete_friendship(
    pool: &PgPool,
    user_low: i32,
    user_high: i32,
) -> Result<bool, sqlx::Error> {
    let res = sqlx::query(
        r#"
        DELETE FROM friendship
        WHERE user_low = $1 AND user_high = $2
    "#,
    )
    .bind(user_low)
    .bind(user_high)
    .execute(pool)
    .await?;
    match res.rows_affected() {
        0 => Ok(false),
        _ => Ok(true),
    }
}

pub async fn manage_friend_request(
    pool: &PgPool,
    id: i32,
    status: Status,
) -> Result<(), sqlx::Error> {
    match status {
        Status::Accepted => {
            let (mut user_low, mut user_high): (i32, i32) = sqlx::query_as(
                r#"
                UPDATE friend_request
                SET status = 'accepted'
                WHERE id = $1
                RETURNING user_from, user_to
            "#,
            )
            .bind(id)
            .fetch_one(pool)
            .await?;
            if user_low > user_high {
                std::mem::swap(&mut user_low, &mut user_high);
            }
            sqlx::query(
                r#"
                INSERT INTO friendship
                (user_low, user_high) VALUES ($1, $2)
            "#,
            )
            .bind(user_low)
            .bind(user_high)
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
