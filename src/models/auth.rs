#[derive(Debug)]
pub enum LoginError {
    Db(sqlx::Error),
    Jwt(jsonwebtoken::errors::Error),
    UserNotFound,
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::Db(e) => write!(f, "DB error: {}", e),
            LoginError::Jwt(e) => write!(f, "JWT error: {}", e),
            LoginError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl From<sqlx::Error> for LoginError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => LoginError::UserNotFound,
            other => LoginError::Db(other),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for LoginError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        LoginError::Jwt(value)
    }
}
