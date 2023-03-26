use crate::{
    authorization::Claims,
    models::user_model::{CreateUserDTO, LoginUserDTO, UserBody, UserEntity},
    services::profile_service,
    validation::ValidatedRequest,
    AppState,
};
use crate::{models::user_model::UpdateUserDTO, services::user_service};
use crate::{validation::CustomError, Result};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};

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

async fn login_user(
    state: Extension<AppState>,
    ValidatedRequest(data): ValidatedRequest<LoginUserDTO>,
) -> Result<Json<UserBody>> {
    let user = user_service::login_user(data, &state.db).await?;

    Ok(Json(UserBody {
        id: user.id,
        name: user.name,
        email: user.email,
        token: Claims::new(user.id).to_jwt(&state)?,
    }))
}

async fn get_current_user(state: Extension<AppState>, claims: Claims) -> Result<Json<UserBody>> {
    let user = profile_service::get_user(claims.sub, &state.db).await?;
    match user {
        Some(user) => Ok(Json(UserBody {
            id: user.id,
            name: user.name,
            email: user.email,
            token: Claims::new(user.id).to_jwt(&state)?,
        })),
        None => Err(CustomError::NotFound),
    }
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
        .route("/", get(get_current_user))
        .route("/me", get(get_current_user))
        .route("/create", post(create_user))
        .route("/login", post(login_user))
        .route("/update/:id", patch(update_user))
        .route("/delete/:id", delete(delete_user))
}

pub fn route() -> Router {
    Router::new().nest("/users", real_route())
}
