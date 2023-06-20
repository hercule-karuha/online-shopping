use axum::{
    routing::{get, post},
    Router, Server,
};

use axum_sessions::{async_session::MemoryStore, SessionLayer};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

use account_handlers::*;
use file_handlers::*;
use store_handlers::*;

pub mod account_handlers;
pub mod error_return;
pub mod file_handlers;
pub mod model;
pub mod schema;
pub mod store_handlers;

#[tokio::main]
async fn main() {
    let pool = create_db_pool();

    let store = MemoryStore::new();
    let secret = b"66666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666"; // MUST be at least 64 bytes!
    let session_layer = SessionLayer::new(store, secret)
        .with_same_site_policy(axum_sessions::SameSite::Lax)
        .with_secure(false);

    let app = Router::new()
        .route("/api/user/register", post(register))
        .route("/api/user/login", post(login))
        .route("/api/store/newStore", post(new_store))
        .route("/api/user/getUserInfo", get(get_user_info))
        .route("/api/file/uploadImage", post(upload_image))
        .route("/api/file/getImage/:image_path", get(get_image))
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
