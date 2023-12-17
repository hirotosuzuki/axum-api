use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new().allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
}
