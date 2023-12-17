// https://docs.rs/tower-http/0.5.0/tower_http/cors/index.html
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new().allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
}
