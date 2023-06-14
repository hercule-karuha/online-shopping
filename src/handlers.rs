use axum::{
    extract::{Json, State},
    response,
};
use serde_json::{json, Value};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use axum_macros::debug_handler;
use diesel::insert_into;
use diesel::prelude::*;


#[debug_handler]
pub async fn register(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<Value>,
) -> response::Json<Value> {
    use crate::schema::users::dsl::*;
    let conn = &mut pool.get().unwrap();
    let user_accont = payload.get("userName").and_then(Value::as_str).unwrap();
    let user_password = payload.get("password").and_then(Value::as_str).unwrap();
    let sex = payload.get("sex").and_then(Value::as_str);
    let user_gender = match sex.unwrap() {
        "1" => "男".to_string(),
        "2" => "女".to_string(),
        _ => String::new(),
    };
    let id = insert_into(users)
        .values((
            account.eq(user_accont),
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
        Err(error) => {
            // 插入失败
            response::Json(json!({
                "code": 500,
                "msg": "注册失败",
                "data": {
                    "userId": error.to_string()
                }
            }))
        }
    }
}