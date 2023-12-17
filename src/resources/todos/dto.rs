use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Serialize)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, max = 100, message = "Todo name is required"))]
    pub name: String,
}

pub type UpdateTodoRequest = CreateTodoRequest;
