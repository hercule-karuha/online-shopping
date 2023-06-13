use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use serde_json::Value;
use  online_shoppig::*;

#[tokio::main] 
async fn main() {
    let connection = &mut establish_connection();
}
