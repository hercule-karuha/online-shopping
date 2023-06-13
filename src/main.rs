use axum::{
    extract::{Extension, Json},
    response,
    routing::post,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;
pub mod requests;
pub mod responses;
pub mod inserts;

#[tokio::main]
async fn main() {
    let pool = create_db_pool();
}

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}

async fn register(
    Json(payload): Json<requests::RegisterRequest>,
    pool: Extension<Pool<ConnectionManager<PgConnection>>>,
) -> response::Json<responses::ApiResponse<responses::UserId>> {
    let conn = pool.get().await?;
    
}
