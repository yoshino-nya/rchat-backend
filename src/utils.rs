pub fn avatar_url(base_url: &str, avatar: &String) -> String {
    format!(
        "{}/uploads/avatars/{}/{}.jpg",
        base_url,
        &avatar[0..2],
        avatar
    )
}
