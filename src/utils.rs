use uuid::Uuid;

use crate::models::session::ChatSessionType;

pub fn avatar_url(base_url: &str, avatar: &String) -> String {
    format!(
        "{}/uploads/avatars/{}/{}.jpg",
        base_url,
        &avatar[0..2],
        avatar
    )
}

pub fn avatar_url_from_uuid(base_url: &str, avatar: Option<Uuid>) -> String {
    let s = match avatar {
        None => "8f6ff5fc-b610-4a4f-8a24-f544418a18ee".to_string(),
        Some(avatar) => avatar.to_string(),
    };
    avatar_url(base_url, &s)
}

pub fn session_avatar_from_uuid(
    base_url: &str,
    avatar: Option<Uuid>,
    session_type: ChatSessionType,
) -> String {
    match session_type {
        ChatSessionType::Private => avatar_url_from_uuid(base_url, avatar),
        ChatSessionType::Group => match avatar {
            Some(uuid) => avatar_url(base_url, &uuid.to_string()),
            None => avatar_url(
                base_url,
                &"fe934bd5-6a81-401c-8668-bf2667df45bf".to_string(),
            ),
        },
    }
}
