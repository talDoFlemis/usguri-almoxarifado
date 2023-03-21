use crate::models::user_model::{CreateUserDTO, UpdateUserDTO, User};
use anyhow::Result;

pub async fn get_all_users(state: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(state)
        .await?;
    Ok(users)
}

pub async fn create_user(user: CreateUserDTO, state: &sqlx::Pool<sqlx::Postgres>) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name) VALUES ($1) RETURNING *",
        user.name
    )
    .fetch_one(state)
    .await?;
    Ok(user)
}

pub async fn get_user(id: i32, state: &sqlx::Pool<sqlx::Postgres>) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * from users WHERE id = $1", id)
        .fetch_one(state)
        .await?;
    Ok(user)
}

pub async fn update_user(
    id: i32,
    data: UpdateUserDTO,
    state: &sqlx::Pool<sqlx::Postgres>,
) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET name = $1 WHERE id = $2 RETURNING *",
        data.name,
        id
    )
    .fetch_one(state)
    .await?;
    Ok(user)
}

pub async fn delete_user(id: i32, state: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(state)
        .await?;

    Ok(())
}
