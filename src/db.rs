use crate::models::{CreatePerson, Person, UpdatePerson};
use axum::{http::StatusCode, Json};
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone)]
pub struct PostgresDB {
    pub conn: PgPool,
}

extern crate dotenv;
use dotenv::dotenv;

impl PostgresDB {
    pub async fn init() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL .env invalid");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_connection_str)
            .await
            .expect("cannot connect to database");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS people (
                id SERIAL NOT NULL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                location VARCHAR(2),
                title VARCHAR(255)
            );"#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { conn: pool })
    }

    pub async fn create_person(
        &self,
        user: CreatePerson,
    ) -> Result<(StatusCode, Json<String>), sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO people (name, location, title) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(user.name)
        .bind(user.location)
        .bind(user.title)
        .fetch_one(&self.conn)
        .await?;

        Ok((
            StatusCode::CREATED,
            Json(format!("Successfully created person #{}", row.0)),
        ))
    }

    pub async fn get_person(&self, id: i32) -> Result<(StatusCode, Json<Person>), sqlx::Error> {
        let fetched_person = sqlx::query_as::<_, Person>(
            "SELECT id, name, location, title FROM people WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.conn)
        .await?;

        Ok((StatusCode::OK, Json(fetched_person)))
    }

    pub async fn update_person(
        &self,
        id: i32,
        user: UpdatePerson,
    ) -> Result<(StatusCode, Json<String>), sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            "UPDATE people SET location = $1, title = $2 WHERE id = $3 RETURNING id",
        )
        .bind(&user.location)
        .bind(&user.title)
        .bind(id)
        .fetch_one(&self.conn)
        .await?;

        Ok((
            StatusCode::OK,
            Json(format!(
                "Successfully update person #{} to location '{}' and title '{}'",
                row.0, user.location, user.title
            )),
        ))
    }

    pub async fn delete_person(&self, id: i32) -> Result<(StatusCode, Json<String>), sqlx::Error> {
        let row: (i32,) = sqlx::query_as("DELETE FROM people WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_one(&self.conn)
            .await?;

        Ok((
            StatusCode::OK,
            Json(format!("Successfully deleted user #{}", row.0)),
        ))
    }

    pub async fn get_all_people(&self) -> Result<(StatusCode, Json<Vec<Person>>), sqlx::Error> {
        let fetched_people =
            sqlx::query_as::<_, Person>("SELECT id, name, location, title FROM people")
                .fetch_all(&self.conn)
                .await?;

        Ok((StatusCode::OK, Json(fetched_people)))
    }
}
