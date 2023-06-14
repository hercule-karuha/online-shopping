use axum::{
    routing::post,
    Router, Server,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;
use handlers::*;

pub mod schema;
pub mod handlers;

#[tokio::main]
async fn main() {
    let pool = create_db_pool();
    let app = Router::new()
        .route("/api/register", post(register))
        .with_state(pool);
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


