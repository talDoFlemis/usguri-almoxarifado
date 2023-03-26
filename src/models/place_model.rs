use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePlaceDTO {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePlaceDTO {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}
