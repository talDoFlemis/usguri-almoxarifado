use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
}
