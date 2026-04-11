use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}
