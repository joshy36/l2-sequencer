use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct AuthError {
    error: String,
    code: String,
}

pub async fn auth_middleware(req: Request, next: Next) -> Response {
    println!("Test");
    let auth_header = match req.headers().get(header::AUTHORIZATION) {
        Some(header) => header.to_str().unwrap_or(""),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    error: "Missing Authorization header".to_string(),
                    code: "AUTH_MISSING".to_string(),
                }),
            )
                .into_response();
        }
    };

    if !is_valid_token(auth_header) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "Invalid auth token".to_string(),
                code: "UNAUTHORIZED".to_string(),
            }),
        )
            .into_response();
    }

    next.run(req).await
}

fn is_valid_token(auth_header: &str) -> bool {
    let expected_token = env::var("AUTH_TOKEN").unwrap_or_else(|_| "default_token".to_string());
    println!("Expected token: {} ", expected_token);
    auth_header
        .strip_prefix("Bearer ")
        .map(|token| token.trim() == expected_token)
        .unwrap_or(false)
}
