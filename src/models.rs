use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct CreatePerson {
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdatePerson {
    pub location: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct GetPerson {
    pub id: i32,
}
