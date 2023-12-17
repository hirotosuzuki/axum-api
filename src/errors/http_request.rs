use axum::{
    async_trait,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

// https://github.com/thanipro/Axum-Rust-Rest-Api-Template/blob/main/src/error/request_error.rs#L19
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

// ValidatedRequest構造体に対してFromRequestトレイトを実装する
#[async_trait]
impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    // HTTPステータスコードとエラーメッセージのタプルを表す型
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Payload"))?;

        // TODO embedded error details
        data.validate()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Payload"))?;

        Ok(Self(data))
    }
}
