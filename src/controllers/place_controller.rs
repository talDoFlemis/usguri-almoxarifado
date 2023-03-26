use crate::{
    models::place_model::{CreatePlaceDTO, PlaceEntity, UpdatePlaceDTO},
    services::place_service,
    validation::{CustomError, ValidatedRequest},
    AppState, Result,
};
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};

async fn get_all(state: Extension<AppState>) -> Result<Json<Vec<PlaceEntity>>> {
    let places = place_service::get_all_places(&state.db).await?;

    Ok(Json(places))
}

async fn get_place(state: Extension<AppState>, Path(id): Path<i32>) -> Result<Json<PlaceEntity>> {
    let place = place_service::get_place(&state.db, id).await?;

    match place {
        Some(place) => Ok(Json(place)),
        None => Err(CustomError::NotFound),
    }
}

async fn create_place(
    state: Extension<AppState>,
    ValidatedRequest(data): ValidatedRequest<CreatePlaceDTO>,
) -> Result<Json<PlaceEntity>> {
    let place = place_service::create_place(&state.db, data).await?;

    Ok(Json(place))
}

async fn update_place(
    state: Extension<AppState>,
    ValidatedRequest(data): ValidatedRequest<UpdatePlaceDTO>,
) -> Result<Json<PlaceEntity>> {
    let place = place_service::update_place(&state.db, data).await?;

    Ok(Json(place))
}

async fn delete_place(state: Extension<AppState>, Path(id): Path<i32>) -> Result<StatusCode> {
    place_service::delete_place(&state.db, id).await?;
    Ok(StatusCode::OK)
}

fn real_route() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/all", get(get_all))
        .route("/:id", get(get_place))
        .route("/create", post(create_place))
        .route("update/:id", patch(update_place))
        .route("/delete/:id", delete(delete_place))
}

pub fn route() -> Router {
    Router::new().nest("/place", real_route())
}
