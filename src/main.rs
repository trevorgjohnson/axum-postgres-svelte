mod api;
mod db;
mod error;
mod models;

use api::{create_person, delete_person, get_all_people, get_person, update_person};
use axum::routing::{get, get_service};
use db::PostgresDB;
use error::internal_error;
use tower_http::services::ServeDir;

extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let webpage_dir = std::env::var("WWW_DIST").unwrap();
    let serve_dir = get_service(ServeDir::new(webpage_dir)).handle_error(internal_error);

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
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
