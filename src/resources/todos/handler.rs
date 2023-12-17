use std::sync::Arc;

use crate::{
    entities::prelude::{Todo, TodoActiveModel},
    errors::{api_error::ApiResult, http_request::ValidatedRequest},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, Set, TransactionTrait};

use super::dto::{CreateTodoRequest, UpdateTodoRequest};

pub async fn index(State(state): State<Arc<AppState>>) -> ApiResult<impl IntoResponse> {
    let todos = Todo::find().all(&state.db).await.unwrap();
    Ok(Json(todos))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(payload): ValidatedRequest<CreateTodoRequest>,
) -> ApiResult<impl IntoResponse> {
    let transaction = state.db.begin().await?;

    let todo = TodoActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(payload.name),
    }
    .insert(&transaction)
    .await?;

    transaction.commit().await?;

    Ok(Json(todo))
}

// https://www.sea-ql.org/SeaORM/docs/basic-crud/update/
pub async fn update(
    State(state): State<Arc<AppState>>,
    ValidatedRequest(payload): ValidatedRequest<UpdateTodoRequest>,
) -> ApiResult<impl IntoResponse> {
    let mut target: TodoActiveModel = Todo::find_by_id(2)
        .one(&state.db)
        .await
        .unwrap()
        .unwrap()
        .into();
    target.name = Set(payload.name);
    let todo = target.update(&state.db).await.unwrap();
    Ok(Json(todo))
}

// https://www.sea-ql.org/SeaORM/docs/basic-crud/delete/
pub async fn delete(State(state): State<Arc<AppState>>) -> ApiResult<impl IntoResponse> {
    let response = Todo::delete_by_id(1).exec(&state.db).await.unwrap();
    if response.rows_affected == 0 {
        return Ok(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}
