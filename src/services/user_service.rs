use crate::models::user_model::User;
use anyhow::Result;

pub async fn get_all_users(state: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(state)
        .await?;
    Ok(users)
}

pub async fn create_user(state: &sqlx::Pool<sqlx::Postgres>) -> Result<User> {
    let user = sqlx::query_as!(User, "INSERT INTO users (name) VALUES ('test') RETURNING *")
        .fetch_one(state)
        .await?;
    Ok(user)
}

// pub async fn delete_user(state: &sqlx::Pool<sqlx::Postgres>) -> _ {
//     let res = sqlx::query!("DELETE FROM users WHERE name = test").await;
// }
