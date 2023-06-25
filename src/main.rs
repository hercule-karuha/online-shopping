use axum::{
    extract::DefaultBodyLimit,
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
use product_handlers::*;
use purchase_handlers::*;
use store_handlers::*;

pub mod account_handlers;
pub mod error_return;
pub mod file_handlers;
pub mod model;
pub mod product_handlers;
pub mod purchase_handlers;
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
        .route("/api/file/getImage/*image_path", get(get_image))
        .route("/api//file/getAvatar/:id", get(get_avatar))
        .route("/api/file/getProductCover/:id", get(get_products_cover))
        .route("/api/file/getStoreCover/:id", get(get_store_cover))
        .route("/api/product/newProduct", post(new_product))
        .route("/api/user/editUserinfo", post(edit_user))
        .route("/api/product/editProduct", post(edit_product))
        .route("/api/store/getStoreInfo/:id", get(get_store_info))
        .route("/api/product/getProduct/:id", get(get_product_info))
        .route("/api/store/editStore", post(edit_store))
        .route("/api/store/getStoreProductList", post(get_product_list))
        .route("/api/purchase/addShoppingCart", post(add_shopping_cart))
        .with_state(pool)
        .layer(session_layer)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10));

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
