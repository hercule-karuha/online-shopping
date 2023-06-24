use axum::{
    extract::{Multipart, Path, State},
    response::Json,
};

use axum_sessions::extractors::ReadableSession;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{insert_into, update};
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

    let path = "images/store_cover/".to_string() + &s_info.as_ref().unwrap().store_id.to_string();
    println!("{}", path);
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
    session: ReadableSession,
    Path(id): Path<String>,
) -> Json<Value> {
    use crate::schema::stores::dsl::*;

    match session.get::<i32>("id") {
        Some(uid) => uid,
        None => {
            return no_login_error();
        }
    };

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
