use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserBody {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate)]
#[allow(unused_mut)]
pub struct LoginUserDTO {
    pub email: String,
    pub password: String,
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
