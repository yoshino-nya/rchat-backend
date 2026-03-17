// services/auth.rs
use crate::models::user::User;
use sqlx::PgPool;

pub struct AuthService;

impl AuthService {
    pub async fn register(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO "user" (username, password)
            VALUES ($1, $2) 
            "#,
            user.username,
            user.password
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    pub async fn login(pool: &PgPool, user: &User) -> Result<bool, sqlx::Error> {
        let is_exist: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM \"user\" WHERE username = $1 AND password = $2)",
        )
        .bind(&user.username)
        .bind(&user.password)
        .fetch_one(pool)
        .await?;
        // fetch_one 返回的第一行，fetch_all 全部，fetch_optional 第一行 or None
        Ok(is_exist)
    }
}
