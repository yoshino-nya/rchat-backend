use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::avatar_url;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: Option<Uuid>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub avatar: String,
}

impl User {
    pub fn to_response(&self, base_url: &str) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.username.clone(),
            avatar: match self.avatar {
                Some(avatar) => avatar_url(base_url, &avatar.to_string()),
                None => avatar_url(
                    base_url,
                    &"8f6ff5fc-b610-4a4f-8a24-f544418a18ee".to_string(),
                ),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}
