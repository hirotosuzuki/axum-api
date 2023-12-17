use std::sync::Arc;

use crate::resources::todos::handler::{create, delete, index, update};
use crate::AppState;
use axum::{
    routing::{get, patch},
    Router,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/todos", get(index).post(create))
        .route("/todos/:id", patch(update).delete(delete))
}
