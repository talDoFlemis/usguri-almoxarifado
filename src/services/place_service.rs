use crate::{models::place_model::UpdatePlaceDTO, Result};
use crate::{
    models::place_model::{CreatePlaceDTO, PlaceEntity},
    validation::ResultExt,
};

pub async fn get_all_places(db: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<PlaceEntity>> {
    let places = sqlx::query_as!(PlaceEntity, "SELECT * FROM places")
        .fetch_all(db)
        .await?;

    Ok(places)
}

pub async fn get_place(db: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<Option<PlaceEntity>> {
    let place = sqlx::query_as!(PlaceEntity, "SELECT * FROM places WHERE id = $1", id)
        .fetch_optional(db)
        .await?;

    Ok(place)
}

pub async fn create_place(
    db: &sqlx::Pool<sqlx::Postgres>,
    data: CreatePlaceDTO,
) -> Result<PlaceEntity> {
    let place = sqlx::query_as!(
        PlaceEntity,
        "INSERT INTO places (name, description, image) VALUES ($1, $2, $3) RETURNING *",
        data.name,
        data.description,
        data.image
    )
    .fetch_one(db)
    .await
    .on_constraint("places_name_key", "name already taken")?;

    Ok(place)
}

pub async fn update_place(
    db: &sqlx::Pool<sqlx::Postgres>,
    data: UpdatePlaceDTO,
) -> Result<PlaceEntity> {
    let place = sqlx::query_as!(
        PlaceEntity,
        "UPDATE places SET name = $1, description = $2, image = $3 WHERE id = $4 RETURNING *",
        data.name,
        data.description,
        data.image,
        data.id
    )
    .fetch_one(db)
    .await
    .on_constraint("places_name_key", "name already taken")?;

    Ok(place)
}

pub async fn delete_place(db: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM places WHERE id = $1", id)
        .execute(db)
        .await?;

    Ok(())
}
