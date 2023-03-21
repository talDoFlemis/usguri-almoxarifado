use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
}
