use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RelationshipResponse {
    pub is_friend: bool,
}
