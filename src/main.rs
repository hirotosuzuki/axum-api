use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    // curl localhost:3000/api/v1/todos | jq .
    // curl -X POST localhost:3000/api/v1/todos | jq .
    // curl -X PATCH localhost:3000/api/v1/todos/1 | jq .
    // curl -X DELETE localhost:3000/api/v1/todos/1 | jq .
    let api = Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete));
    let app = Router::new().nest("/api/v1", api);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn todos_index() -> impl IntoResponse {
    let todo1 = json!({
        "id": 1,
        "name": "todo1",
    });
    let todo2 = json!({
        "id": 2,
        "name": "todo2",
    });
    Json(vec![todo1, todo2])
}

async fn todos_create() -> impl IntoResponse {
    let todo = json!({
        "id": 1,
        "name": "todo1",
    });
    (StatusCode::CREATED, Json(todo))
}

async fn todos_update() -> Result<impl IntoResponse, StatusCode> {
    let todo = json!({
        "id": 1,
        "name": "todo1",
    });
    Ok(Json(todo))
}

async fn todos_delete() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, "")
}
