use axum::{
    extract::{Json, State},
    response,
};

use axum_sessions::extractors::ReadableSession;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{dsl::count_star, prelude::*};
use serde_json::{from_value, json, Value};

use crate::model::*;

use crate::error_return::*;

pub async fn search_product(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::products::dsl::*;

    let request_data: SearchInfo = match from_value(payload) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON: {}", error);
            return response::Json(json!({
                "code": 400,
                "message": "Invalid JSON format"
            }));
        }
    };

    let p_size = match request_data.pageSize.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let p_no = match request_data.pageNo.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let st_id = match request_data.storeId.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    let total = products
        .select(count_star())
        .filter(name.like(format!("%{}%", request_data.keyword)))
        .first::<i64>(conn)
        .unwrap();

    let query = products
        .select((product_id, name, price))
        .offset((p_size * (p_no - 1)) as i64)
        .limit(p_size.into())
        .into_boxed();

    let filt_query = match st_id {
        0 => query.filter(
            name.like(format!("%{}%", request_data.keyword))
                .and(delete_product.ne(1)),
        ),
        _ => query.filter(
            name.like(format!("%{}%", request_data.keyword))
                .and(store_id.eq(st_id).and(delete_product.ne(1))),
        ),
    };

    let results = match filt_query.get_results::<(i32, Option<String>, Option<f64>)>(conn) {
        Ok(res_vec) => res_vec,
        Err(_) => {
            return server_error();
        }
    };

    let mut value_vec: Vec<Value> = Vec::new();

    for prod_info in results.iter() {
        if let (p_id, Some(p_name), Some(p_pric)) = prod_info {
            value_vec.push(json!({
                "productId":p_id.to_string(),
                "productName":p_name,
                "price":p_pric.to_string(),
            }));
        }
    }

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "pageSize":p_size,
            "pageNo":p_no+1,
            "pageCount":((total / p_size as i64) + (total % p_size as i64> 0) as i64).to_string(),
            "total":total.to_string(),
            "list":value_vec
        },
    }))
}

pub async fn search_orders(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::orders::dsl::*;
    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    let request_data: SearchInfo = match from_value(payload) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON: {}", error);
            return response::Json(json!({
                "code": 400,
                "message": "Invalid JSON format"
            }));
        }
    };

    let p_size = match request_data.pageSize.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let p_no = match request_data.pageNo.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    let total = orders
        .select(count_star())
        .filter(
            product_name
                .like(format!("%{}%", request_data.keyword))
                .and(user_id.eq(usr_id)),
        )
        .first::<i64>(conn)
        .unwrap();

    let results = orders
        .select(OrderInfo::as_select())
        .offset((p_size * (p_no - 1)) as i64)
        .limit(p_size.into())
        .filter(
            product_name
                .like(format!("%{}%", request_data.keyword))
                .and(user_id.eq(usr_id)),
        )
        .get_results(conn)
        .unwrap();

    let mut value_vec: Vec<Value> = Vec::new();

    for u_order in results.iter() {
        value_vec.push(json!({
            "orderId":u_order.order_id.to_string(),
            "productId":u_order.product_id.unwrap().to_string(),
            "productName":u_order.product_name.clone().unwrap(),
            "price":u_order.total_price.unwrap().to_string(),
            "num":u_order.quantity.unwrap().to_string(),
            "time":u_order.purchase_time.unwrap().to_string(),
            "sendAddress":u_order.store_address.clone().unwrap(),
            "receiveAddress":u_order.user_address.clone().unwrap(),
            "phone":u_order.user_phone.clone().unwrap(),
        }))
    }

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "pageSize":p_size,
            "pageNo":p_no+1,
            "pageCount":((total / p_size as i64) + (total % p_size as i64> 0) as i64).to_string(),
            "total":total.to_string(),
            "list":value_vec
        },
    }))
}

pub async fn search_sales(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::orders::dsl::*;
    match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    match session.get::<i32>("type") {
        Some(1) => {}
        _ => {
            println!("wrong user_type");
            return parameter_error();
        }
    };
    let st_id = match session.get::<i32>("store_id") {
        Some(s_id) => s_id,
        _ => {
            return server_error();
        }
    };

    let request_data: SearchInfo = match from_value(payload) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON: {}", error);
            return response::Json(json!({
                "code": 400,
                "message": "Invalid JSON format"
            }));
        }
    };

    let p_size = match request_data.pageSize.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let p_no = match request_data.pageNo.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    let total = orders
        .select(count_star())
        .filter(
            product_name
                .like(format!("%{}%", request_data.keyword))
                .and(store_id.eq(st_id)),
        )
        .first::<i64>(conn)
        .unwrap();

    let results = orders
        .select(OrderInfo::as_select())
        .offset((p_size * (p_no - 1)) as i64)
        .limit(p_size.into())
        .filter(
            product_name
                .like(format!("%{}%", request_data.keyword))
                .and(store_id.eq(st_id)),
        )
        .get_results(conn)
        .unwrap();

    let mut value_vec: Vec<Value> = Vec::new();

    for u_order in results.iter() {
        value_vec.push(json!({
            "orderId":u_order.order_id.to_string(),
            "productId":u_order.product_id.unwrap().to_string(),
            "productName":u_order.product_name.clone().unwrap(),
            "price":u_order.total_price.unwrap().to_string(),
            "num":u_order.quantity.unwrap().to_string(),
            "time":u_order.purchase_time.unwrap().to_string(),
            "sendAddress":u_order.store_address.clone().unwrap(),
            "receiveAddress":u_order.user_address.clone().unwrap(),
            "phone":u_order.user_phone.clone().unwrap(),
        }))
    }

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "pageSize":p_size,
            "pageNo":p_no+1,
            "pageCount":((total / p_size as i64) + (total % p_size as i64> 0) as i64).to_string(),
            "total":total.to_string(),
            "list":value_vec
        },
    }))
}
