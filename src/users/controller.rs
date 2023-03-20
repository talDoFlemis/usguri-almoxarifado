use super::service;
use crate::models::user_model::User;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use sqlx::PgPool;

async fn get_all(state: Extension<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = service::get_all_users(&state.0).await.unwrap();
    Ok(Json(users))
}

async fn create_user(state: Extension<PgPool>) -> Result<Json<User>, StatusCode> {
    let user = service::create_user(&state.0).await.unwrap();
    Ok(Json(user))
}

// async fn delete_user(state: Extension<PgPool>) -> Result<StatusCode, StatusCode> {
//     let user = service::delete_user(&state.0).await.unwrap();
//     Ok(StatusCode::OK)
// }

fn real_route() -> Router {
    Router::new()
        .route("/all", get(get_all))
        .route("/create", post(create_user))
}

pub fn route() -> Router {
    Router::new().nest("/users", real_route())
}
