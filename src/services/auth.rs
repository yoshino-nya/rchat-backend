// services/auth.rs
use crate::models::user::{LoginRequest, RegisterRequest};
use sqlx::{PgPool, Row};

pub struct AuthService;

impl AuthService {
    pub async fn register(pool: &PgPool, req: &RegisterRequest) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO "user" (username, password)
            VALUES ($1, $2)
        "#,
        )
        .bind(&req.username)
        .bind(&req.password)
        .execute(pool)
        .await?;
        Ok(())
    }
    pub async fn login(pool: &PgPool, req: &LoginRequest) -> Result<i32, sqlx::Error> {
        let res: i32 = sqlx::query(
            r#"
            SELECT id FROM "user" WHERE username = $1 AND password = $2
        "#,
        )
        .bind(&req.username)
        .bind(&req.password)
        .fetch_one(pool)
        .await?
        .try_get("id")?;
        Ok(res)
    }
}
