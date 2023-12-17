mod entities;
mod errors;
mod resources;

use std::sync::Arc;

use crate::resources::todos;
use axum::Router;
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let db = Database::connect("sqlite://sample.db").await.unwrap();
    let state = Arc::new(AppState { db });

    // curl localhost:3000/api/v1/todos | jq .
    // curl -X POST localhost:3000/api/v1/todos -H "Content-Type: application/json" -d '{"name": "sample"}' | jq .
    // curl -X PATCH localhost:3000/api/v1/todos/1 | jq .
    // curl -X DELETE localhost:3000/api/v1/todos/1 | jq .
    let api = Router::new()
        .merge(todos::route::routes())
        .with_state(state);

    let app = Router::new().nest("/api/v1", api);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub struct AppState {
    db: DatabaseConnection,
}
