// services/auth.rs
use crate::models::{
    auth::LoginError,
    user::{LoginRequest, RegisterRequest},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct AuthService;

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user_id
    pub exp: usize,
}

fn generate_token(user_id: i32, secret: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims { sub: user_id, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct LoginResponse {
    pub user_id: i32,
    pub username: String,
    pub token: String,
}
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
    pub async fn login(
        pool: &PgPool,
        req: &LoginRequest,
        secret: &String,
    ) -> Result<LoginResponse, LoginError> {
        let (user_id, username): (i32, String) = sqlx::query_as(
            r#"
            SELECT id, username FROM "user" WHERE username = $1 AND password = $2
        "#,
        )
        .bind(&req.username)
        .bind(&req.password)
        .fetch_one(pool)
        .await?;
        Ok(LoginResponse {
            user_id,
            username,
            token: generate_token(user_id, secret)?,
        })
    }
}
