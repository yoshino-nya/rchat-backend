use std::sync::Arc;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::state::AppState;
use crate::{models::response::ApiResponse, services::auth::Claims};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();
    if path == "/api/login"
        || path == "/api/register"
        || path.starts_with("/ws")
        || path.starts_with("/uploads")
    {
        return next.run(req).await;
    }

    let auth_header = match req.headers().get("authorization") {
        Some(h) => match h.to_str() {
            Ok(val) => val,
            Err(e) => {
                tracing::error!(path = %path, error = %e, "Authorization header invalid UTF-8");
                return (
                    StatusCode::UNAUTHORIZED,
                    ApiResponse::<()>::err("Invalid authorization header encoding".to_string()),
                )
                    .into_response();
            }
        },
        None => {
            tracing::error!(path = %path, "Missing authorization header");
            return (
                StatusCode::UNAUTHORIZED,
                ApiResponse::<()>::err("Authentication token not provided".to_string()),
            )
                .into_response();
        }
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => {
            tracing::error!(path = %path, "Authorization header missing Bearer prefix");
            return (
                StatusCode::UNAUTHORIZED,
                ApiResponse::<()>::err("Invalid auth header".to_string()),
            )
                .into_response();
        }
    };

    let secret = &state.config.jwt_secret;
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(_) => next.run(req).await,
        Err(e) => {
            tracing::error!(path = %path, error = %e, "JWT decode failed");
            (
                StatusCode::UNAUTHORIZED,
                ApiResponse::<()>::err("Invalid token".to_string()),
            )
                .into_response()
        }
    }
}
