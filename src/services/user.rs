use crate::models::user::User;
use axum::Json;
use sqlx::PgPool;
pub struct UserService;

impl UserService {
    pub async fn find_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, sqlx::Error> {
        let res: Option<User> = sqlx::query_as(
            r#"
            SELECT id, username, password FROM "user" WHERE id = $1"#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        match res {
            Some(user) => Ok(user),
            None => Err(sqlx::Error::RowNotFound),
        }
    }
    pub async fn find_user_by_name(pool: &PgPool, username: String) -> Result<User, sqlx::Error> {
        let res: Option<User> = sqlx::query_as(
            r#"
            SELECT id, username, password FROM "user" WHERE username = $1"#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;
        match res {
            Some(user) => Ok(user),
            None => Err(sqlx::Error::RowNotFound),
        }
    }
}
