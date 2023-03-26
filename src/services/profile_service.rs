use crate::models::profile_model::ProfileEntity;
use crate::Result;

pub async fn get_all_users(state: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<ProfileEntity>> {
    let users = sqlx::query_as!(ProfileEntity, "SELECT id, name, email FROM users")
        .fetch_all(state)
        .await?;

    Ok(users)
}

pub async fn get_user(
    id: i32,
    state: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<ProfileEntity>> {
    let user = sqlx::query_as!(
        ProfileEntity,
        "SELECT id, name, email from users WHERE id = $1",
        id
    )
    .fetch_optional(state)
    .await?;
    Ok(user)
}
