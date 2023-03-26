use crate::{
    models::user_model::{CreateUserDTO, UpdateUserDTO, User},
    validation::ResultExt,
};
use crate::{validation::CustomError, Result};
use anyhow::Context;
use argon2::{password_hash::SaltString, PasswordHasher, PasswordVerifier};
use argon2::{Argon2, PasswordHash};
use chrono::DateTime;

pub async fn get_all_users(state: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(state)
        .await?;
    Ok(users)
}

pub async fn create_user(user: CreateUserDTO, state: &sqlx::Pool<sqlx::Postgres>) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *",
        user.name,
        user.email,
        user.password
    )
    .fetch_one(state)
    .await?;
    Ok(user)
}

pub async fn get_user(id: i32, state: &sqlx::Pool<sqlx::Postgres>) -> Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * from users WHERE id = $1", id)
        .fetch_optional(state)
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
        "UPDATE users SET name = $2, email = $3 WHERE id = $1 RETURNING *",
        id,
        data.name,
        data.password,
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
async fn hash_password(password: String) -> Result<String> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Could not hash password: {}", e))?
            .to_string())
    })
    .await
    .context("Panic in generating password hash")??)
}

async fn verify_password(password: String, hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let parsed_hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow::anyhow!("Could not parse password hash: {}", e))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => CustomError::Unauthorized,
                _ => anyhow::anyhow!("Could not verify password: {}", e).into(),
            })?)
    })
    .await
    .context("Panic in verifying password")??)
}
