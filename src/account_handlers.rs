use axum::{
    extract::{Json, Multipart, State},
    response,
};

use axum_macros::debug_handler;

use axum_sessions::extractors::{ReadableSession, WritableSession};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{dsl::count_star, prelude::*};
use diesel::{insert_into, update};
use serde_json::{from_value, json, Value};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use md5;

use crate::error_return::*;

use crate::model::*;

#[debug_handler]
pub async fn register(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,

    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::users::dsl::*;
    let usr_name = match payload.get("userName").and_then(Value::as_str) {
        Some(uname) => uname,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "没有输入用户名",
                "data": null,
            }));
        }
    };
    let sex = match payload.get("gender").and_then(Value::as_str) {
        Some(usex) => usex,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "没有输入性别",
                "data": null,
            }));
        }
    };

    let user_password = match payload["password"].as_str() {
        Some(upassword) => upassword,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "没有输入密码",
                "data": null,
            }));
        }
    };
    if user_password.len() < 8 || user_password.len() > 20 {
        return response::Json(json!({
            "code": 600,
            "msg": "密码必须在8~20位之间",
            "data": null,
        }));
    }

    let user_gender = match sex {
        "1" => 1,
        "2" => 2,
        _ => 0,
    };

    let conn = &mut pool.get().unwrap();

    let id = insert_into(users)
        .values((
            user_name.eq(usr_name),
            password.eq(user_password),
            gender.eq(user_gender),
            user_type.eq(0),
        ))
        .returning(user_id)
        .execute(conn);
    match id {
        Ok(_) => {
            // 插入成功
            response::Json(json!({
                "code": 200,
                "msg": "注册成功",
                "data": null,
            }))
        }
        Err(_) => {
            // 插入失败
            response::Json(json!({
                "code": 600,
                "msg": "用户名不唯一",
                "data": null,
            }))
        }
    }
}

pub async fn login(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    mut session: WritableSession,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::stores::dsl;
    use crate::schema::stores::dsl::*;
    use crate::schema::users::dsl::*;

    let u_name = match payload["userName"].as_str() {
        Some(uname) => uname,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "没有输入用户名",
                "data": null,
            }));
        }
    };
    let md5_password = match payload["password"].as_str() {
        Some(upassword) => upassword,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "没有输入密码",
                "data": null,
            }));
        }
    };

    let conn = &mut pool.get().unwrap();

    let user = users
        .filter(user_name.eq(u_name))
        .select(User::as_select())
        .first(conn);

    match user {
        Err(_) => response::Json(json!({
            "code": 600,
            "msg": "尚未注册",
            "data": null,
        })),
        Ok(usr) => {
            if md5_password == format!("{:x}", md5::compute(usr.password.unwrap())) {
                response::Json(json!({
                    "code": 600,
                    "msg": "密码错误",
                    "data": null,
                }))
            } else {
                session
                    .insert("id", usr.user_id)
                    .expect("cannot store value");
                session
                    .insert("type", usr.user_type.as_ref())
                    .expect("cannot store value");
                session
                    .insert("gender", usr.gender)
                    .expect("cannot store value");
                session
                    .insert("name", usr.user_name.as_ref().unwrap())
                    .expect("cannot store value");

                let mut sto_id = 0;

                if usr.user_type.unwrap() == 1 {
                    let st_id: Result<i32, diesel::result::Error> = stores
                        .select(store_id)
                        .filter(dsl::user_id.eq(usr.user_id))
                        .first(conn);
                    sto_id = st_id.unwrap();
                    session
                        .insert("store_id", sto_id)
                        .expect("cannot store value");
                }

                response::Json(json!({
                    "code": 200,
                    "msg": "登陆成功",
                    "data": {
                        "userId": usr.user_id.to_string(),
                        "userName": usr.user_name.unwrap().to_string(), //用户昵称
                        "sex": usr.gender.unwrap().to_string(), //性别
                        "userType": usr.user_type.unwrap().to_string(), //是否是商家 0不是 1 是
                        "storeId": sto_id.to_string()
                    }
                }))
            }
        }
    }
}

pub async fn get_user_info(session: WritableSession) -> response::Json<Value> {
    match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    match session.get::<i32>("type").unwrap() {
        0 => response::Json(json!({
            "code": 200,
            "msg": "请求成功",
            "data": {
                "userId": session.get::<i32>("id").unwrap(),
                "userName": session.get::<String>("name").unwrap(), //用户昵称
                "sex": session.get::<i32>("gender").unwrap(), //性别
                "userType": session.get::<i32>("type").unwrap() //是否是商家 0不是 1 是
            }
        })),
        1 => response::Json(json!({
            "code": 200,
            "msg": "请求成功",
            "data": {
                "userId": session.get::<i32>("id").unwrap(),
                "userName": session.get::<String>("name").unwrap(), //用户昵称
                "sex": session.get::<i32>("gender").unwrap(), //性别
                "userType": session.get::<i32>("type").unwrap(), //是否是商家 0不是 1 是
                "storeId":session.get::<i32>("store_id").unwrap()
            }
        })),
        _ => parameter_error(),
    }
}

pub async fn get_user_detail(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
) -> response::Json<Value> {
    use crate::schema::users::dsl::*;
    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    match users
        .select(UserDetail::as_select())
        .filter(user_id.eq(usr_id))
        .first(conn)
    {
        Ok(usr_info) => response::Json(json!({
            "code": 200,
            "msg": "登陆成功",
            "data": {
                "userId": usr_info.user_id.to_string(),
                "userName": usr_info.user_name.unwrap().to_string(), //用户昵称
                "sex": usr_info.gender.unwrap().to_string(), //性别
                "userType": usr_info.user_type.unwrap().to_string(), //是否是商家 0不是 1 是
                "address":match usr_info.address {
                    Some(addr) =>addr,
                    None => "".to_string()
                },
                "phone":match usr_info.phone {
                    Some(pho) =>pho,
                    None => "".to_string()
                }
            }
        })),
        Err(_) => server_error(),
    }
}

pub async fn edit_user(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    mut multipart: Multipart,
) -> response::Json<Value> {
    use crate::schema::users::dsl::*;
    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };

    let mut edit_usr = UpdateUser::new(usr_id);
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap().to_string().as_str() {
            "userName" => {
                edit_usr.user_name = Some(field.text().await.unwrap().to_owned());
            }
            "gender" => match field.text().await.unwrap().as_str() {
                "0" => {
                    edit_usr.gender = Some(0);
                }

                "1" => {
                    edit_usr.gender = Some(1);
                }

                "2" => {
                    edit_usr.gender = Some(2);
                }
                _ => {
                    return parameter_error();
                }
            },
            "phone" => {
                edit_usr.phone = Some(field.text().await.unwrap().to_owned());
            }
            "address" => {
                edit_usr.address = Some(field.text().await.unwrap().to_owned());
            }
            "avatar" => {
                let data = field.bytes().await.unwrap();
                if !data.is_empty() {
                    let path = "images/avatar/".to_string() + &usr_id.to_string();
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

    match update(users)
        .set(edit_usr)
        .filter(user_id.eq(usr_id))
        .execute(conn)
    {
        Ok(_) => response::Json(json!({
            "code": 200,
            "msg": "修改成功",
            "data": null
        })),
        Err(_) => server_error(),
    }
}

pub async fn get_shopping_cart(
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

    let total = match shopping_carts
        .select(count_star())
        .filter(user_id.eq(usr_id))
        .first::<i64>(conn)
    {
        Ok(all) => all,
        Err(_) => {
            return server_error();
        }
    };

    let carts = match shopping_carts
        .select(CartInfo::as_select())
        .offset((p_size * (p_no - 1)) as i64)
        .limit(p_size.into())
        .filter(user_id.eq(usr_id))
        .get_results(conn)
    {
        Ok(rev_vec) => rev_vec,
        Err(_) => {
            return server_error();
        }
    };

    let mut resvec: Vec<Value> = Vec::new();

    for cart_info in carts.iter() {
        match dsl::products
            .select((dsl::name, dsl::price))
            .filter(dsl::product_id.eq(cart_info.product_id))
            .first::<(Option<String>, Option<f64>)>(conn)
        {
            Ok((prod_name, prod_price)) => resvec.push(json!({
                "productId":cart_info.product_id.to_string(),
                "productName":prod_name,
                "price":prod_price.unwrap().to_string(),
                "num":cart_info.quantity.unwrap().to_string(),
            })),
            Err(_) => {
                return server_error();
            }
        };
    }

    response::Json(json!({
        "code": 200,
        "msg": "修改成功",
        "data": {
        "pageSize":p_size,
        "pageNo":p_no,
        "pageCount":((total / p_size as i64) + (total % p_size as i64> 0) as i64).to_string(),
        "total":total.to_string(),
        "list":resvec
        },
    }))
}

pub async fn get_order_list(
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
        .filter(user_id.eq(usr_id))
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
        .filter(user_id.eq(usr_id))
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

pub async fn get_order_detail(
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

    let conn = &mut pool.get().unwrap();

    let oid_str = match payload["orderId"].as_str() {
        Some(id) => id.to_owned(),
        None => {
            return parameter_error();
        }
    };

    let oid = match oid_str.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return parameter_error();
        }
    };

    let u_order = match orders
        .select(OrderInfo::as_select())
        .filter(order_id.eq(oid))
        .first(conn)
    {
        Ok(res) => res,
        Err(_) => {
            return server_error();
        }
    };

    response::Json(json!({
        "code": 200,
        "msg": "修改成功",
        "data":{
            "orderId":u_order.order_id.to_string(),
            "storeId":u_order.store_id.unwrap().to_string(),
            "productId":u_order.product_id.unwrap().to_string(),
            "productName":u_order.product_name.clone().unwrap(),
            "price":u_order.total_price.unwrap().to_string(),
            "num":u_order.quantity.unwrap().to_string(),
            "sendAddress":u_order.store_address.clone().unwrap(),
            "receiveAddress":u_order.user_address.clone().unwrap(),
            "time":u_order.purchase_time.unwrap().to_string(),
        }
    }))
}
