use axum::{
    extract::{Json, State},
    response,
};

use std::time::SystemTime;

use axum_sessions::extractors::ReadableSession;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{insert_into, update};
use serde_json::{from_value, json, Value};

use crate::model::*;

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

    let request_data: CartRequestData = match from_value(payload) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON: {}", error);
            return response::Json(json!({
                "code": 400,
                "message": "Invalid JSON format"
            }));
        }
    };

    let conn = &mut pool.get().unwrap();

    for prod in request_data.list.iter() {
        let prod_id = match prod.productId.parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                return parameter_error();
            }
        };

        let num: i32 = match prod.num.parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                return parameter_error();
            }
        };
        match insert_into(shopping_carts)
            .values((
                product_id.eq(prod_id),
                user_id.eq(usr_id),
                quantity.eq(num),
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
    use crate::schema::products::dsl;

    println!("purchase something");

    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    let request_data: PurRequestData = match from_value(payload) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON: {}", error);
            return response::Json(json!({
                "code": 400,
                "message": "Invalid JSON format"
            }));
        }
    };

    let usr_address = request_data.address;
    let usr_phone = request_data.phone;
    let list = request_data.list;

    let conn = &mut pool.get().unwrap();

    for prod in list.iter() {
        let prod_id = match prod.productId.parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                return parameter_error();
            }
        };

        let num = match prod.num.parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                return parameter_error();
            }
        };

        let prod_info = update(dsl::products.find(prod_id))
            .set((
                dsl::stock.eq(dsl::stock - num),
                dsl::sales.eq(dsl::sales + num),
            ))
            .returning((dsl::price, dsl::store_id, dsl::store_address))
            .get_result::<ProductOrderInfo>(conn)
            .unwrap();

        match insert_into(orders)
            .values((
                user_id.eq(usr_id),
                product_id.eq(prod_id),
                purchase_time.eq(SystemTime::now()),
                total_price.eq(prod_info.price.unwrap() * (num as f64)),
                quantity.eq(num),
                user_phone.eq(usr_phone.as_str()),
                user_address.eq(usr_address.as_str()),
                store_address.eq(prod_info.store_address.unwrap()),
                store_id.eq(prod_info.store_id.unwrap()),
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
