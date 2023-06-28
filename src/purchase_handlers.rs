use axum::{
    extract::{Json, State},
    response,
};

use axum_sessions::extractors::ReadableSession;
use chrono::Local;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{delete, insert_into, update};
use serde_json::{from_value, json, Value};

use crate::model::*;

use crate::error_return::*;

pub async fn add_shopping_cart(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::products::dsl;
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

        match dsl::products
            .select(dsl::delete_product)
            .filter(dsl::product_id.eq(prod_id))
            .first::<Option<i32>>(conn)
            .unwrap()
        {
            Some(1) => {
                return parameter_error();
            }
            _ => {}
        }

        match insert_into(shopping_carts)
            .values((product_id.eq(prod_id), user_id.eq(usr_id), quantity.eq(num)))
            .execute(conn)
        {
            Ok(_) => {}
            Err(_) => {
                match update(shopping_carts.filter(user_id.eq(usr_id).and(product_id.eq(prod_id))))
                    .set(quantity.eq(quantity + num))
                    .execute(conn)
                {
                    Ok(_) => {}
                    Err(_) => {
                        return server_error();
                    }
                }
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
    use crate::schema::shopping_carts;

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

        match dsl::products
            .select(dsl::delete_product)
            .filter(dsl::product_id.eq(prod_id))
            .first::<Option<i32>>(conn)
            .unwrap()
        {
            Some(1) => {
                return parameter_error();
            }
            _ => {}
        }

        let prod_info = update(dsl::products.find(prod_id))
            .set((
                dsl::stock.eq(dsl::stock - num),
                dsl::sales.eq(dsl::sales + num),
            ))
            .returning((dsl::price, dsl::store_id, dsl::store_address, dsl::name))
            .get_result::<ProductOrderInfo>(conn)
            .unwrap();

        match insert_into(orders)
            .values((
                user_id.eq(usr_id),
                product_id.eq(prod_id),
                purchase_time.eq(Local::now().naive_local()),
                product_name.eq(prod_info.name.unwrap()),
                total_price.eq(prod_info.price.unwrap() * (num as f64)),
                quantity.eq(num),
                user_phone.eq(usr_phone.as_str()),
                user_address.eq(usr_address.as_str()),
                store_address.eq(prod_info.store_address.unwrap()),
                store_id.eq(prod_info.store_id.unwrap()),
            ))
            .execute(conn)
        {
            Ok(_) => delete(
                shopping_carts::dsl::shopping_carts.filter(
                    shopping_carts::dsl::user_id
                        .eq(usr_id)
                        .and(shopping_carts::dsl::product_id.eq(prod_id)),
                ),
            )
            .execute(conn)
            .expect("cannot delete"),
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

pub async fn edit_shopping_cart(
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

        match num {
            0 => {
                match delete(shopping_carts.filter(user_id.eq(usr_id).and(product_id.eq(prod_id))))
                    .execute(conn)
                {
                    Ok(_) => {}
                    Err(_) => {
                        return server_error();
                    }
                };
            }
            _ => {
                match update(shopping_carts.filter(user_id.eq(usr_id).and(product_id.eq(prod_id))))
                    .set(quantity.eq(num))
                    .execute(conn)
                {
                    Ok(_) => {}
                    Err(_) => {
                        return server_error();
                    }
                }
            }
        }
    }

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": null,
    }))
}
