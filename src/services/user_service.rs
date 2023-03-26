use crate::{
    models::user_model::{CreateUserDTO, LoginUserDTO, ProfileEntity, UpdateUserDTO, UserEntity},
    validation::ResultExt,
};
use crate::{validation::CustomError, Result};
use anyhow::Context;
use argon2::{password_hash::SaltString, PasswordHasher, PasswordVerifier};
use argon2::{Argon2, PasswordHash};

pub async fn get_all_users(state: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<UserEntity>> {
    let users = sqlx::query_as!(UserEntity, "SELECT * FROM users")
        .fetch_all(state)
        .await?;
    Ok(users)
}

pub async fn create_user(
    user: CreateUserDTO,
    state: &sqlx::Pool<sqlx::Postgres>,
) -> Result<ProfileEntity> {
    let pass_hash = hash_password(user.password).await?;

    let user_id = sqlx::query_scalar!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id",
        user.name,
        user.email,
        pass_hash
    )
    .fetch_one(state)
    .await
    .on_constraint("users_email_key", "email already taken")?;

    Ok(ProfileEntity {
        id: user_id,
        name: user.name,
        email: user.email,
    })
}

pub async fn login_user(
    req: LoginUserDTO,
    state: &sqlx::Pool<sqlx::Postgres>,
) -> Result<ProfileEntity> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT * FROM users WHERE email = $1",
        req.email
    )
    .fetch_optional(state)
    .await?
    .ok_or(CustomError::Unauthorized)?;

    verify_password(req.password, user.password).await?;

    Ok(ProfileEntity {
        id: user.id,
        name: user.name,
        email: user.email,
    })
}

pub async fn get_user(id: i32, state: &sqlx::Pool<sqlx::Postgres>) -> Result<Option<UserEntity>> {
    let user = sqlx::query_as!(UserEntity, "SELECT * from users WHERE id = $1", id)
        .fetch_optional(state)
        .await?;
    Ok(user)
}

pub async fn update_user(
    id: i32,
    data: UpdateUserDTO,
    state: &sqlx::Pool<sqlx::Postgres>,
) -> Result<UserEntity> {
    let user = sqlx::query_as!(
        UserEntity,
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
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Could not hash password: {}", e))?
            .to_string())
    })
    .await
    .context("Panic in generating password hash")?
}

async fn verify_password(password: String, hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let parsed_hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow::anyhow!("Could not parse password hash: {}", e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => CustomError::Unauthorized,
                _ => anyhow::anyhow!("Could not verify password: {}", e).into(),
            })
    })
    .await
    .context("Panic in verifying password")?
}
