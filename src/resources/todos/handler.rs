use std::sync::Arc;

use crate::{
    entities::prelude::{Todo, TodoActiveModel},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, Set};

pub async fn todos_index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos = Todo::find().all(&state.db).await.unwrap();
    Json(todos)
}

pub async fn todos_create(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = TodoActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set("created todo".to_owned()),
    };
    todo.insert(&state.db).await.unwrap();
    Ok(StatusCode::CREATED)
}

// https://www.sea-ql.org/SeaORM/docs/basic-crud/update/
pub async fn todos_update(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
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
pub async fn todos_delete(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let response = Todo::delete_by_id(1).exec(&state.db).await.unwrap();
    if response.rows_affected == 0 {
        return StatusCode::NOT_FOUND;
    }
    StatusCode::NO_CONTENT
}
