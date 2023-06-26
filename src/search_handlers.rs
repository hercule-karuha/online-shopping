use axum::{
    extract::{Json, State},
    response,
};

use axum_sessions::extractors::ReadableSession;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde_json::{from_value, json, Value};

use crate::model::*;

use crate::error_return::*;

pub async fn search_product(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::products::dsl::*;
    match session.get::<i32>("id") {
        Some(_) => {}
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

    let st_id = match request_data.storeId.parse::<i32>() {
        Ok(sz) => sz,
        Err(_) => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    let query = products
        .select((product_id, name, price))
        .offset((p_size * (p_no - 1)) as i64)
        .limit(p_size.into())
        .into_boxed();

    let filt_query = match st_id {
        0 => query.filter(name.like(format!("%{}%", request_data.keyword))),
        _ => query.filter(
            name.like(format!("%{}%", request_data.keyword))
                .and(store_id.eq(st_id)),
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
                "name":p_name,
                "price":p_pric.to_string(),
            }));
        }
    }

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "pageNo":p_no.to_string(),
            "pageSize":p_size.to_string(),
            "list":value_vec,
        },
    }))
}
