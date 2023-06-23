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
    response::Json(json!({
        "code": 200,
        "msg": "登陆成功",
        "data": {
            "userId": session.get::<i32>("id").unwrap(),
            "userName": session.get::<String>("name").unwrap(), //用户昵称
            "sex": session.get::<i32>("gender").unwrap(), //性别
            "userType": session.get::<i32>("type").unwrap() //是否是商家 0不是 1 是
        }
    }))
}
