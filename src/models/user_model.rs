use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    pub password: String,
}
