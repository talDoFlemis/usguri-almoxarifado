use crate::services::user_service;
use crate::{
    models::user_model::{CreateUserDTO, User},
    validation::ValidatedRequest,
};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use sqlx::PgPool;

async fn get_all(state: Extension<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = user_service::get_all_users(&state.0).await.unwrap();
    Ok(Json(users))
}

async fn get_user(state: Extension<PgPool>, Path(id): Path<i32>) -> Result<Json<User>, StatusCode> {
    let user = user_service::get_user(id, &state.0).await.unwrap();
    Ok(Json(user))
}

async fn create_user(
    state: Extension<PgPool>,
    ValidatedRequest(data): ValidatedRequest<CreateUserDTO>,
) -> Result<Json<User>, StatusCode> {
    let user = user_service::create_user(data, &state.0).await.unwrap();
    Ok(Json(user))
}

async fn delete_user(
    state: Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    user_service::delete_user(id, &state.0).await.unwrap();
    Ok(StatusCode::OK)
}

fn real_route() -> Router {
    Router::new()
        .route("/all", get(get_all))
        .route("/:id", get(get_user))
        .route("/create", post(create_user))
        .route("/delete/:id", delete(delete_user))
}

pub fn route() -> Router {
    Router::new().nest("/users", real_route())
}
