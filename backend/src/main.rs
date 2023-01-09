mod api;
mod db;
mod error;
mod models;

use api::{create_person, delete_person, get_all_people, get_person, update_person};
use axum::routing::get;
use db::PostgresDB;

extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = PostgresDB::init().await.expect("Pool initalization failed");

    let app = axum::Router::new()
        .route(
            "/person",
            get(get_person)
                .post(create_person)
                .put(update_person)
                .delete(delete_person),
        )
        .route("/people", get(get_all_people))
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("error binding to axum server");
}
