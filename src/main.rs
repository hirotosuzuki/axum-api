mod entity;

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use entity::prelude::{Todo, TodoActiveModel};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, Set};

#[tokio::main]
async fn main() {
    let db = Database::connect("sqlite://sample.db").await.unwrap();
    let state = Arc::new(AppState { db });

    // curl localhost:3000/api/v1/todos | jq .
    // curl -X POST localhost:3000/api/v1/todos | jq .
    // curl -X PATCH localhost:3000/api/v1/todos/1 | jq .
    // curl -X DELETE localhost:3000/api/v1/todos/1 | jq .
    let api = Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        .with_state(state);

    let app = Router::new().nest("/api/v1", api);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn todos_index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos = Todo::find().all(&state.db).await.unwrap();
    Json(todos)
}

async fn todos_create(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let todo = TodoActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set("created todo".to_owned()),
    };
    todo.insert(&state.db).await.unwrap();
    Ok(StatusCode::CREATED)
}

// https://www.sea-ql.org/SeaORM/docs/basic-crud/update/
async fn todos_update(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    let mut target: TodoActiveModel = Todo::find_by_id(2)
        .one(&state.db)
        .await
        .unwrap()
        .unwrap()
        .into();
    target.name = Set("updated todo".to_owned());
    let todo = target.update(&state.db).await.unwrap();
    Ok(Json(todo))
}

// https://www.sea-ql.org/SeaORM/docs/basic-crud/delete/
async fn todos_delete(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let response = Todo::delete_by_id(1).exec(&state.db).await.unwrap();
    if response.rows_affected == 0 {
        return StatusCode::NOT_FOUND;
    }
    StatusCode::NO_CONTENT
}

struct AppState {
    db: DatabaseConnection,
}
