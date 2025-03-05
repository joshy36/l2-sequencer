use http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers(Any)
}
