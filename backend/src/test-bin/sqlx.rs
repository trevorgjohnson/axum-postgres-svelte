#![allow(unused)]

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    FromRow, Row,
};

#[derive(Debug, FromRow)]
struct Person {
    id: i32,
    name: String,
    location: String,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // ======================= //
    // ESTABLISH CONNECT TO PG //
    // ======================= //
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost/postgres")
        .await?;

    // ====================== //
    // QUERY AND CREATE TABLE //
    // ====================== //
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

    // ======================== //
    // QUERY AND INSERT INTO PG //
    // ======================== //
    let row: (i32,) = sqlx::query_as(
        "INSERT INTO people (name, location, title) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind("Thomas Dingle")
    .bind("SD")
    .bind("mr. fucker")
    .fetch_one(&pool)
    .await?;

    // ============================ //
    // QUERY AND FORMAT INTO STRING //
    // ============================ //
    let rows = sqlx::query("SELECT * FROM people").fetch_all(&pool).await?;
    let str_result = rows
        .iter()
        .map(|p| {
            format!(
                "{} - {} from {} | {}",
                p.get::<i32, _>("id"),
                p.get::<String, _>("name"),
                p.get::<String, _>("location"),
                p.get::<String, _>("title")
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("\n== SELECT * FROM PEOPLE ==\n{}", str_result);

    // =================================== //
    // QUERY AND MANUALLY CAST INTO STRUCT //
    // =================================== //
    let select_query = sqlx::query("SELECT id, name, location, title FROM people");
    let people: Vec<Person> = select_query
        .map(|row: PgRow| Person {
            id: row.get("id"),
            name: row.get("name"),
            location: row.get("location"),
            title: row.get("title"),
        })
        .fetch_all(&pool)
        .await?;

    println!("\n=== SELECT people FROM query.map ===\n{:?}", people);

    // ============================ //
    // QUERY_AS TO CAST INTO STRUCT //
    // ============================ //
    let select_query = sqlx::query_as::<_, Person>("SELECT id, name, location ,title FROM people");
    let people: Vec<Person> = select_query.fetch_all(&pool).await?;

    println!("\n=== SELECT people FROM query.map ===\n{:?}", people);

    Ok(())
}
