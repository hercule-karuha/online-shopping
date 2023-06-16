use axum::{routing::post, Router, Server};

use axum_sessions::{async_session::MemoryStore, SessionLayer};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use account_handlers::*;
use std::env;

pub mod account_handlers;
pub mod model;
pub mod schema;

#[tokio::main]
async fn main() {
    let pool = create_db_pool();

    let store = MemoryStore::new();
    let secret = b"66666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666"; // MUST be at least 64 bytes!
    let session_layer =
        SessionLayer::new(store, secret).with_same_site_policy(axum_sessions::SameSite::None);

    let app = Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .with_state(pool)
        .layer(session_layer);

    let addr = "0.0.0.0:3000".parse().unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}
