use crate::{
    authorization::Claims,
    models::user_model::{CreateUserDTO, UserBody, UserEntity},
    validation::ValidatedRequest,
    AppState,
};
use crate::{
    models::user_model::{ProfileEntity, UpdateUserDTO},
    services::user_service,
};
use crate::{validation::CustomError, Result};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};

async fn get_all(state: Extension<AppState>) -> Result<Json<Vec<UserEntity>>> {
    let users = user_service::get_all_users(&state.db).await?;
    Ok(Json(users))
}

async fn get_user(state: Extension<AppState>, Path(id): Path<i32>) -> Result<Json<UserEntity>> {
    let user = user_service::get_user(id, &state.db).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(CustomError::NotFound),
    }
}

async fn create_user(
    state: Extension<AppState>,
    ValidatedRequest(data): ValidatedRequest<CreateUserDTO>,
) -> Result<Json<UserBody>> {
    let user = user_service::create_user(data, &state.db).await?;
    Ok(Json(UserBody {
        id: user.id,
        name: user.name,
        email: user.email,
        token: Claims::new(user.id).to_jwt(&state)?,
    }))
}

async fn update_user(
    state: Extension<AppState>,
    Path(id): Path<i32>,
    ValidatedRequest(data): ValidatedRequest<UpdateUserDTO>,
) -> Result<Json<UserEntity>> {
    let user = user_service::update_user(id, data, &state.db).await?;

    Ok(Json(user))
}

async fn delete_user(state: Extension<AppState>, Path(id): Path<i32>) -> Result<StatusCode> {
    user_service::delete_user(id, &state.db).await?;
    Ok(StatusCode::OK)
}

fn real_route() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/all", get(get_all))
        .route("/:id", get(get_user))
        .route("/create", post(create_user))
        .route("/update/:id", patch(update_user))
        .route("/delete/:id", delete(delete_user))
}

pub fn route() -> Router {
    Router::new().nest("/users", real_route())
}
