use axum::{routing::post, Router, Server};

use axum_sessions::{
    async_session::MemoryStore, extractors::WritableSession, PersistencePolicy, SessionLayer,
};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use handlers::*;
use std::env;

pub mod handlers;
pub mod schema;

#[tokio::main]
async fn main() {
    let pool = create_db_pool();

    let store = MemoryStore::new();
    let secret = b"66666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666"; // MUST be at least 64 bytes!
    let session_layer = SessionLayer::new(store, secret);

    let app = Router::new()
        .route("/api/register", post(register))
        .with_state(pool)
        .route("/login", post(test_login))
        .route("/session", post(test_session))
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
