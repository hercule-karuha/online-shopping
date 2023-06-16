use axum::{
    extract::{Json, State},
    response,
};

use axum_macros::debug_handler;

use axum_sessions::extractors::WritableSession;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde_json::{json, Value};

use md5;

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
                "msg": "请求参数错误",
                "data": null,
            }));
        }
    };
    let sex = match payload.get("sex").and_then(Value::as_str) {
        Some(usex) => usex,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "请求参数错误",
                "data": null,
            }));
        }
    };

    let user_password = match payload["password"].as_str() {
        Some(upassword) => upassword,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "请求参数错误",
                "data": null,
            }));
        }
    };
    if user_password.len() < 8 || user_password.len() > 20 {
        return response::Json(json!({
            "code": 600,
            "msg": "请求参数错误",
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
        ))
        .returning(user_id)
        .execute(conn);
    match id {
        Ok(inserted_id) => {
            // 插入成功
            response::Json(json!({
                "code": 200,
                "msg": "注册成功",
                "data": {
                    "userId": inserted_id.to_string()
                }
            }))
        }
        Err(_) => {
            // 插入失败
            response::Json(json!({
                "code": 600,
                "msg": "请求参数错误",
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
    use crate::schema::users::dsl::*;

    let name = match payload["userName"].as_str() {
        Some(uname) => uname,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "请求参数错误",
                "data": null,
            }));
        }
    };
    let md5_password = match payload["password"].as_str() {
        Some(upassword) => upassword,
        None => {
            return response::Json(json!({
                "code": 600,
                "msg": "请求参数错误",
                "data": null,
            }));
        }
    };

    let conn = &mut pool.get().unwrap();

    let user = users
        .filter(user_name.eq(name))
        .select(User::as_select())
        .first(conn);

    match user {
        Err(_) => response::Json(json!({
            "code": 600,
            "msg": "请求参数错误",
            "data": null,
        })),
        Ok(usr) => {
            if md5_password == format!("{:x}", md5::compute(usr.password.unwrap())) {
                response::Json(json!({
                    "code": 600,
                    "msg": "请求参数错误",
                    "data": null,
                }))
            } else {
                session
                    .insert("id", usr.user_id)
                    .expect("cannot store value");
                session
                    .insert("type", usr.user_type)
                    .expect("cannot store value");
                
                response::Json(json!({
                    "code": 200,
                    "msg": "请求成功",
                    "data": {
                        "userId": usr.user_id.to_string(),
                        "userName": usr.user_name.unwrap().to_string(), //用户昵称
                        "sex": usr.gender.unwrap().to_string(), //性别
                        "userType": usr.user_type.unwrap().to_string() //是否是商家 0不是 1 是
                    }
                }))
            }
        }
    }
}
