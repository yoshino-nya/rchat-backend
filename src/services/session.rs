use sqlx::PgPool;
use uuid::Uuid;

use crate::models::session::ChatSessionType;

pub async fn find_session_id(
    pool: &PgPool,
    user_id1: i32,
    user_id2: i32,
) -> Result<Uuid, sqlx::Error> {
    let res: Option<Uuid> = sqlx::query_scalar(
        r#"
        SELECT sm.session_id
        FROM chat_session_members sm
        JOIN chat_session s ON sm.session_id = s.uuid
        WHERE s.type = 'private'
            AND sm.user_id IN ($1, $2)
        GROUP BY sm.session_id
        HAVING
            COUNT(CASE WHEN sm.user_id IN ($1, $2) THEN 1 END) = 2
    "#,
    )
    .bind(user_id1)
    .bind(user_id2)
    .fetch_optional(pool)
    .await
    .unwrap();
    match res {
        None => create_session(pool, vec![user_id1, user_id2], ChatSessionType::Private).await,
        Some(uid) => Ok(uid),
    }
}

pub async fn create_session(
    pool: &PgPool,
    users: Vec<i32>,
    session_type: ChatSessionType,
) -> Result<Uuid, sqlx::Error> {
    let uid = Uuid::new_v4();
    let name = users
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("、");
    sqlx::query(
        r#"
        INSERT INTO chat_session (type, uuid, name)
        VALUES ($1, $2, $3)
    "#,
    )
    .bind(session_type)
    .bind(uid)
    .bind(name)
    .execute(pool)
    .await?;
    for user in users {
        sqlx::query(
            r#"
            INSERT INTO chat_session_members
            (session_id, user_id)
            VALUES ($1, $2)
        "#,
        )
        .bind(uid)
        .bind(user)
        .execute(pool)
        .await?;
    }
    Ok(uid)
}
