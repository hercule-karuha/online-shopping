use axum::response::Json;
use serde_json::{json, Value};

pub fn parameter_error() -> Json<Value> {
    Json(json!({
        "code": 600,
        "msg": "请求参数错误",
        "data": null,
    }))
}

pub fn no_login_error() -> Json<Value> {
    Json(json!({
        "code": 900,
        "msg": "登录超时或未登录",
        "data": null,
    }))
}

pub fn server_error() -> Json<Value> {
    Json(json!({
        "code": 500,
        "msg": "服务器错误",
        "data": null,
    }))
}

