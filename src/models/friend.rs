use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug)]
pub struct Friendship {
    pub user_low: i32,
    pub user_high: i32,
    pub created_time: DateTime<Utc>,
}

pub enum Status {
    Accepted,
    Rejected,
    Pending,
}

#[derive(Debug, Deserialize)]
pub struct FriendRequest {
    pub user_from: i32,
    pub user_to: i32,
}
