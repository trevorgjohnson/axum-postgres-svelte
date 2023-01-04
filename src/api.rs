use crate::db::PostgresDB;
use crate::error::CustomError;
use crate::models::{CreatePerson, GetPerson, UpdatePerson};

use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
};

pub async fn create_person(
    State(pool): State<PostgresDB>,
    Json(payload): Json<CreatePerson>,
) -> impl IntoResponse {
    let new_user = CreatePerson {
        name: payload.name,
        location: payload.location,
        title: payload.title,
    };

    pool.create_person(new_user)
        .await
        .map_err(|e| CustomError::from(e).into_response())
}

pub async fn get_person(
    State(pool): State<PostgresDB>,
    query_string: Query<GetPerson>,
) -> impl IntoResponse {
    pool.get_person(query_string.0.id)
        .await
        .map_err(|e| CustomError::from(e).into_response())
}

pub async fn update_person(
    State(pool): State<PostgresDB>,
    query_string: Query<GetPerson>,
    Json(payload): Json<UpdatePerson>,
) -> impl IntoResponse {
    let update_user = UpdatePerson {
        location: payload.location,
        title: payload.title,
    };

    pool.update_person(query_string.0.id, update_user)
        .await
        .map_err(|e| CustomError::from(e).into_response())
}

pub async fn delete_person(
    State(pool): State<PostgresDB>,
    query_string: Query<GetPerson>,
) -> impl IntoResponse {
    pool.delete_person(query_string.0.id)
        .await
        .map_err(|e| CustomError::from(e).into_response())
}

pub async fn get_all_people(State(pool): State<PostgresDB>) -> impl IntoResponse {
    pool.get_all_people()
        .await
        .map_err(|e| CustomError::from(e).into_response())
}
