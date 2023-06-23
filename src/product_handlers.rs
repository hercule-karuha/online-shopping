use axum::{
    extract::{Multipart, State},
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

pub async fn new_product(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    mut multipart: Multipart,
) -> Json<Value> {
    use crate::schema::products::dsl::*;
    match session.get::<i32>("id") {
        Some(_) => {}
        None => {
            return no_login_error();
        }
    };
    match session.get::<i32>("type") {
        Some(1) => {}
        _ => {
            return parameter_error();
        }
    };
    let st_id = match session.get::<i32>("store_id") {
        Some(s_id) => s_id,
        _ => {
            return server_error();
        }
    };

    let mut new_prod = NewProduct::new(Some(st_id));
    let mut cover: Vec<u8> = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap().to_string().as_str() {
            "name" => {
                new_prod.name = Some(field.text().await.unwrap().to_owned());
            }
            "description" => {
                new_prod.description = Some(field.text().await.unwrap().to_owned());
            }
            "detail_images" => {
                new_prod.detail_images = Some(field.text().await.unwrap().to_owned());
            }
            "price" => {
                if let Ok(pric) = field.text().await.unwrap().parse::<f64>() {
                    new_prod.price = Some(pric);
                } else {
                    println!("wrong price");
                    return parameter_error();
                }
            }
            "stock" => {
                if let Ok(stoc) = field.text().await.unwrap().parse::<i32>() {
                    new_prod.stock = Some(stoc);
                } else {
                    println!("wrong stock");
                    return parameter_error();
                }
            }
            "cover" => {
                cover = field.bytes().await.unwrap().to_vec();
            }
            _ => {
                return parameter_error();
            }
        }
    }

    let conn = &mut pool.get().unwrap();

    let pro_id = insert_into(products)
        .values(&new_prod)
        .returning(product_id)
        .get_result::<i32>(conn);

    let path = "images/products_cover/".to_string() + &pro_id.as_ref().unwrap().to_string();
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
            "product_id": pro_id.unwrap().to_string(),//用户id
        },
    }))
}

pub async fn edit_product(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    mut multipart: Multipart,
) -> Json<Value> {
    use crate::schema::products::dsl::*;
    match session.get::<i32>("id") {
        Some(_) => {}
        None => {
            return no_login_error();
        }
    };
    match session.get::<i32>("type") {
        Some(1) => {}
        _ => {
            return parameter_error();
        }
    };
    let mut new_prod = NewProduct::new(None);
    let mut prod_id = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap().to_string().as_str() {
            "name" => {
                new_prod.name = Some(field.text().await.unwrap().to_owned());
            }
            "description" => {
                new_prod.description = Some(field.text().await.unwrap().to_owned());
            }
            "detail_images" => {
                new_prod.detail_images = Some(field.text().await.unwrap().to_owned());
            }
            "price" => {
                if let Ok(pric) = field.text().await.unwrap().parse::<f64>() {
                    new_prod.price = Some(pric);
                } else {
                    println!("wrong price");
                    return parameter_error();
                }
            }
            "stock" => {
                if let Ok(stoc) = field.text().await.unwrap().parse::<i32>() {
                    new_prod.stock = Some(stoc);
                } else {
                    println!("wrong stock");
                    return parameter_error();
                }
            }
            "cover" => {
                let data = field.bytes().await.unwrap();
                if !data.is_empty() {
                    let path = "images/products_cover/".to_string() + &prod_id.to_string();
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
            "productId" => {
                if let Ok(id) = field.text().await.unwrap().parse::<i32>() {
                    prod_id = id;
                } else {
                    println!("wrong product_id");
                    return parameter_error();
                }
            }
            _ => {
                return parameter_error();
            }
        }
    }

    let conn = &mut pool.get().unwrap();

    match update(products)
        .set(new_prod)
        .filter(product_id.eq(prod_id))
        .execute(conn)
    {
        Ok(_) => Json(json!({
            "code": 200,
            "msg": "修改成功",
            "data": null
        })),
        Err(_) => server_error(),
    }
}
