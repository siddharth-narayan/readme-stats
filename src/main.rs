use axum::{Router, routing::{get}};
use crate::{langs::languages, repo::repo};

mod langs;
mod repo;
mod world;
mod util;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new()
        .route("/repos/{user}/{repo}", get(repo))
        .route("/languages/{user}", get(languages));

    axum::serve(listener, app).await.unwrap();
}