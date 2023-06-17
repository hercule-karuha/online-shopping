use axum::{
    extract::{Multipart, State},
    response::Json,
};

use axum_sessions::extractors::ReadableSession;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde_json::{json, Value};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::model::*;

use crate::error_return::*;

pub async fn new_store(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    mut multipart: Multipart,
) -> Json<Value> {
    use crate::schema::stores::dsl::*;
    let usr_id = match session.get::<i32>("id") {
        Some(id) => id,
        None => {
            return no_login_error();
        }
    };
    match session.get::<i32>("type") {
        Some(1) => 1,
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
    let cover_type = cover_field.content_type().unwrap().to_string();
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

    if let Some(index) = cover_type.rfind('/') {
        let extension = &cover_type[index..];
        let path = "image/cover".to_string()
            + &s_info.as_ref().unwrap().store_id.to_string()
            + "."
            + extension;
        let mut file = File::create(path).await.expect("file error");
        file.write_all(&cover).await.expect("write error");
    }

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
