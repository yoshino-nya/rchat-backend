use axum::extract::multipart::MultipartError;
use std::fmt;
#[derive(Debug)]
pub enum UploadError {
    Io(std::io::Error),
    Multipart(MultipartError),
    Db(sqlx::Error),
    Nofile,
    InvalidFileType,
}

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UploadError::Io(e) => write!(f, "IO error: {}", e),
            UploadError::Multipart(e) => write!(f, "Multipart error: {}", e),
            UploadError::Db(e) => write!(f, "Database error: {}", e),
            UploadError::Nofile => write!(f, "No file uploaded"),
            UploadError::InvalidFileType => write!(f, "Invalid file type"),
        }
    }
}

impl From<MultipartError> for UploadError {
    fn from(value: MultipartError) -> Self {
        UploadError::Multipart(value)
    }
}

impl From<std::io::Error> for UploadError {
    fn from(value: std::io::Error) -> Self {
        UploadError::Io(value)
    }
}

impl From<sqlx::Error> for UploadError {
    fn from(value: sqlx::Error) -> Self {
        UploadError::Db(value)
    }
}
