use http::{header::HeaderName, HeaderValue, Request};

use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct MyMakeRequestId {}

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let uuid = Uuid::new_v4().to_string();
        let request_id = HeaderValue::from_str(&uuid).unwrap();
        Some(RequestId::new(request_id))
    }
}

pub fn create_request_id_layer() -> SetRequestIdLayer<MyMakeRequestId> {
    let x_request_id = HeaderName::from_static("x-request-id");
    SetRequestIdLayer::new(x_request_id.clone(), MyMakeRequestId::default())
}

pub fn create_propagate_request_id_layer() -> PropagateRequestIdLayer {
    let x_request_id = HeaderName::from_static("x-request-id");
    PropagateRequestIdLayer::new(x_request_id)
}
