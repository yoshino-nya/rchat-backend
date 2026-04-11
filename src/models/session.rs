#[derive(sqlx::Type)]
#[sqlx(type_name = "chat_session_type", rename_all = "lowercase")]
pub enum ChatSessionType {
    Private,
    Group,
}
