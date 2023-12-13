use std::sync::Arc;

use crate::resources::todos::handler::{todos_create, todos_delete, todos_index, todos_update};
use crate::AppState;
use axum::{
    routing::{get, patch},
    Router,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
}
