use axum::{extract::Path, routing::get, Extension, Json, Router};

use crate::Result;
use crate::{
    models::profile_model::ProfileEntity, services::profile_service, validation::CustomError,
    AppState,
};

async fn get_all(state: Extension<AppState>) -> Result<Json<Vec<ProfileEntity>>> {
    let users = profile_service::get_all_users(&state.db).await?;
    Ok(Json(users))
}

async fn get_user(state: Extension<AppState>, Path(id): Path<i32>) -> Result<Json<ProfileEntity>> {
    let user = profile_service::get_user(id, &state.db).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(CustomError::NotFound),
    }
}

fn real_route() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/all", get(get_all))
        .route("/:id", get(get_user))
}

pub fn route() -> Router {
    Router::new().nest("/profile", real_route())
}
