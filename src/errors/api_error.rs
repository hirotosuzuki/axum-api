use axum::{http::StatusCode, response::IntoResponse, Json};

pub struct ApiError {
    status: StatusCode,
    response: Json<serde_json::Value>,
}

pub type ApiResult<T> = anyhow::Result<T, ApiError>;

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(original_error: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            response: axum::Json(serde_json::json!({
                "error": format!("{:#?}", original_error.into())
            })),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.response).into_response()
    }
}
