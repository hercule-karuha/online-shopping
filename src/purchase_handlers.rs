use axum::{
    extract::{Json, State},
    response,
};

use axum_sessions::extractors::ReadableSession;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{insert_into, update};
use serde_json::{json, Value};

use crate::error_return::*;

pub async fn add_shopping_cart(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::shopping_carts::dsl::*;
    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    let prod_list_js: Vec<&Value> = match &payload["list"] {
        Value::Array(ls) => ls.iter().collect(),
        _ => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    for prod in prod_list_js.iter() {
        match insert_into(shopping_carts)
            .values((
                product_id.eq(prod["productId"].to_string().parse::<i32>().unwrap()),
                user_id.eq(usr_id),
                quantity.eq(prod["num"].to_string().parse::<i32>().unwrap()),
            ))
            .execute(conn)
        {
            Ok(_) => {}
            Err(_) => {
                return server_error();
            }
        };
    }

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": null,
    }))
}

pub async fn immediate_purchase(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::orders::dsl::*;
    use crate::schema::products::dsl::*;

    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    let usr_phone = match &payload["phone"].as_str() {
        Some(pho) => pho,
        _ => {
            return parameter_error();
        }
    };

    let usr_address = match &payload["phone"].as_str() {
        Some(addr) => addr,
        _ => {
            return parameter_error();
        }
    };

    let prod_list_js: Vec<&Value> = match &payload["list"] {
        Value::Array(ls) => ls.iter().collect(),
        _ => {
            return parameter_error();
        }
    };

    for prod in prod_list_js.iter() {
        match insert_into(orders)
            .values((
                product_id.eq(prod["productId"].to_string().parse::<i32>().unwrap()),
                user_id.eq(usr_id),
                quantity.eq(prod["num"].to_string().parse::<i32>().unwrap()),
            ))
            .execute(conn)
        {
            Ok(_) => {}
            Err(_) => {
                return server_error();
            }
        };
    }




    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": null,
    }))
}
