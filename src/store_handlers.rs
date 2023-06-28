use axum::{
    extract::{Json, Multipart, Path, State},
    response,
};

use axum_sessions::extractors::{ReadableSession, WritableSession};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{dsl::count_star, prelude::*};
use diesel::{insert_into, update};
use serde_json::{from_value, json, Value};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::model::*;

use crate::error_return::*;

pub async fn new_store(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    mut session: WritableSession,
    mut multipart: Multipart,
) -> Json<Value> {
    use crate::schema::stores::dsl::*;
    use crate::schema::users::dsl;
    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };
    match session.get::<i32>("type") {
        Some(0) => 0,
        _ => {
            return parameter_error();
        }
    };
    let store_name = match multipart.next_field().await {
        Ok(sname) => sname.unwrap().text().await.unwrap(),
        Err(_) => {
            return parameter_error();
        }
    };
    let cover_field = match multipart.next_field().await {
        Ok(cf) => cf,
        Err(_) => {
            return parameter_error();
        }
    };

    let cover_field = cover_field.unwrap();
    let cover = cover_field.bytes().await.unwrap();

    let store_address = match multipart.next_field().await {
        Ok(saddress) => saddress.unwrap().text().await.unwrap(),
        Err(_) => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    let s_info = insert_into(stores)
        .values((
            user_id.eq(usr_id),
            name.eq(store_name),
            address.eq(store_address),
        ))
        .returning(Store::as_returning())
        .get_result(conn);

    update(dsl::users)
        .filter(dsl::user_id.eq(usr_id))
        .set(dsl::user_type.eq(1))
        .execute(conn)
        .expect("update fail");

    session.insert("type", 1).expect("cannot store value");
    session
        .insert("store_id", s_info.as_ref().unwrap().store_id)
        .expect("cannot store value");

    let path = "images/store_cover/".to_string() + &s_info.as_ref().unwrap().store_id.to_string();
    let mut file = match File::create(path).await {
        Ok(fe) => fe,
        Err(error) => {
            println!("{}", error);
            return server_error();
        }
    };
    file.write_all(&cover).await.expect("write error");

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "storeName": s_info.as_ref().unwrap().name, //店铺名称
            "sroreId": s_info.unwrap().store_id.to_string(), //店铺id
            "userId": usr_id.to_string(),//用户id
        },
    }))
}

pub async fn edit_store(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    mut multipart: Multipart,
) -> Json<Value> {
    use crate::schema::stores::dsl::*;

    match session.get::<i32>("id") {
        Some(uid) => uid,
        None => {
            return no_login_error();
        }
    };
    let st_id = match session.get::<i32>("store_id") {
        Some(sid) => sid,
        _ => {
            return parameter_error();
        }
    };

    let mut edit_store = Store::new(st_id);

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap().to_string().as_str() {
            "storeId" => {
                if let Ok(id) = field.text().await.unwrap().parse::<i32>() {
                    if id != st_id {
                        return parameter_error();
                    }
                } else {
                    println!("wrong product_id");
                    return parameter_error();
                }
            }
            "storeName" => {
                edit_store.name = Some(field.text().await.unwrap().to_owned());
            }
            "address" => {
                edit_store.address = Some(field.text().await.unwrap().to_owned());
            }
            "cover" => {
                let data = field.bytes().await.unwrap();
                if !data.is_empty() {
                    let path = "images/store_cover/".to_string() + &st_id.to_string();
                    let mut file = match File::create(path).await {
                        Ok(fe) => fe,
                        Err(error) => {
                            println!("{}", error);
                            return server_error();
                        }
                    };
                    file.write_all(&data).await.expect("write error");
                }
            }
            _ => {
                return parameter_error();
            }
        }
    }
    let conn = &mut pool.get().unwrap();

    match update(stores)
        .set(edit_store)
        .filter(store_id.eq(st_id))
        .execute(conn)
    {
        Ok(_) => Json(json!({
            "code": 200,
            "msg": "请求成功",
            "data": null,
        })),
        Err(_) => server_error(),
    }
}

pub async fn get_store_info(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id): Path<String>,
) -> Json<Value> {
    use crate::schema::stores::dsl::*;

    let st_id = match id.parse::<i32>() {
        Ok(sid) => sid,
        Err(_) => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    let store_res = stores
        .select(Store::as_select())
        .filter(store_id.eq(st_id))
        .first(conn)
        .unwrap();

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "name" : store_res.name.unwrap(),
            "userId" : store_res.user_id.unwrap().to_string(),
            "address" :  store_res.address.unwrap(),
            "storeId" : st_id.to_string(),
        },
    }))
}

pub async fn get_product_list(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    use crate::schema::products::dsl::*;

    let st_id = match payload["storeId"].as_str() {
        Some(id) => id.parse::<i32>().unwrap(),
        None => {
            return parameter_error();
        }
    };
    let order_type = match payload["orderType"].as_str() {
        Some(ot) => ot.parse::<i32>().unwrap(),
        None => {
            return parameter_error();
        }
    };
    let page_no = match payload["pageNo"].as_str() {
        Some(pn) => pn.parse::<i32>().unwrap() - 1,
        None => {
            return parameter_error();
        }
    };
    let page_sz = match payload["pageSize"].as_str() {
        Some(psz) => psz.parse::<i32>().unwrap(),
        None => {
            return parameter_error();
        }
    };

    let mut result_vec: Vec<Value> = Vec::new();

    let conn = &mut pool.get().unwrap();

    let total = products
        .select(count_star())
        .filter(store_id.eq(st_id).and(delete_product.ne(1)))
        .first::<i64>(conn)
        .unwrap();

    let query = products
        .filter(store_id.eq(st_id).and(delete_product.ne(1)))
        .offset((page_no * page_sz).into())
        .limit(page_sz.into());

    let ordered_query = match order_type {
        0 => query.order(product_id.asc()).into_boxed(),
        1 => query.order(product_id.desc()).into_boxed(),
        2 => query.order(sales.asc()).into_boxed(),
        3 => query.order(sales.desc()).into_boxed(),
        4 => query.order(price.asc()).into_boxed(),
        5 => query.order(price.desc()).into_boxed(),
        _ => query.into_boxed(), // 默认排序方式
    };

    let results = ordered_query
        .select(Product::as_select())
        .get_results::<Product>(conn);

    if let Ok(resvec) = results {
        for prod in resvec.iter() {
            result_vec.push(json!({
                "productId": prod.product_id.to_string(),
                "productName": prod.name.clone().unwrap(),
                "price": prod.price.unwrap().to_string(),
                "sales": prod.sales.unwrap().to_string(),
            }))
        }
    }

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "pageSize": page_sz.to_string(), //一页的个数
            "pageNo": (page_no+1).to_string(), //页数
            "pageCount": ((total / page_sz as i64) + (total % page_sz as i64> 0) as i64).to_string(), //总页数
            "total": total.to_string(), //总记录数
            "list":result_vec,
        },
    }))
}

pub async fn get_sale_order(
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
    let mut resvec: Vec<Value> = Vec::new();

    let request_data: PageInfo = match from_value(payload) {
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

    let total = match orders
        .select(count_star())
        .filter(store_id.eq(st_id))
        .first::<i64>(conn)
    {
        Ok(all) => all,
        Err(_) => {
            return server_error();
        }
    };

    let usr_orders = match orders
        .select(OrderInfo::as_select())
        .offset((p_size * (p_no - 1)) as i64)
        .limit(p_size.into()).order(order_id.desc())
        .filter(store_id.eq(st_id))
        .get_results(conn)
    {
        Ok(rev_vec) => rev_vec,
        Err(_) => {
            return server_error();
        }
    };

    for u_order in usr_orders.iter() {
        resvec.push(json!({
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

    response::Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "pageSize":p_size,
            "pageNo":p_no,
            "pageCount":(total / p_size as i64).to_string(),
            "total":total.to_string(),
            "list":resvec
        }
    }))
}
