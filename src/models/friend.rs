use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

#[derive(Debug)]
pub struct Friendship {
    pub user_low: i32,
    pub user_high: i32,
    pub created_time: DateTime<Utc>,
}

#[derive(Type, Debug, Deserialize, Serialize)]
#[sqlx(type_name = "friend_request_status")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Accepted,
    Rejected,
    Pending,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct FriendRequest {
    pub user_from: i32,
    pub user_to: i32,
    pub status: Status,
    pub created_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFriendRequest {
    pub user_from: i32,
    pub user_to: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteFriendshipRequest {
    pub user_a: i32,
    pub user_b: i32,
}

#[derive(Debug, FromRow)]
pub struct FriendInfo {
    pub user_id: i32,
}
