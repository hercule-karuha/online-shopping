use axum::{
    extract::{Json, State},
    response,
};
use axum_macros::debug_handler;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde_json::{json, Value};

#[debug_handler]
pub async fn register(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::users::dsl::*;
    let user_password = payload["password"].as_str().unwrap();
    if user_password.len() < 8 || user_password.len() > 20 {
        return response::Json(json!({
            "code": 500,
            "msg": "密码必须在8~20位之间"
        }));
    }
    let usr_name = payload.get("userName").and_then(Value::as_str).unwrap();
    let sex = payload.get("sex").and_then(Value::as_str);
    let user_gender = match sex.unwrap() {
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
                "code": 500,
                "msg": "用户名不唯一"
            }))
        }
    }
}
