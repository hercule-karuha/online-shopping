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

use tokio::fs::write;
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

    let mut new_prod = ProductIns::new(Some(st_id));
    let mut cover: Vec<u8> = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap().to_string().as_str() {
            "name" => {
                new_prod.name = Some(field.text().await.unwrap().to_owned());
            }
            "description" => {
                new_prod.description = Some(field.text().await.unwrap().to_owned());
            }
            "detailImages" => {
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
            "address" => {
                new_prod.store_address = Some(field.text().await.unwrap().to_owned());
            }
            _ => {
                println!("unknown field name");
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
            "productId": pro_id.unwrap().to_string(),//用户id
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
    let mut new_prod = ProductIns::new(None);
    let mut prod_id = 0;
    new_prod.sales = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap().to_string().as_str() {
            "name" => {
                new_prod.name = Some(field.text().await.unwrap().to_owned());
            }
            "description" => {
                new_prod.description = Some(field.text().await.unwrap().to_owned());
            }
            "detailImages" => {
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
                    println!("get cover of {}", prod_id);
                    let path = "images/products_cover/".to_string() + &prod_id.to_string();

                    if let Err(err) = write(path, &data).await {
                        eprintln!("Failed to write file: {}", err);
                    } else {
                        println!("File overwritten successfully.");
                    }

                    // let mut file = match File::create(path).await {
                    //     Ok(fe) => fe,
                    //     Err(error) => {
                    //         println!("{}", error);
                    //         return server_error();
                    //     }
                    // };
                    // file.write(&data).await.expect("write error");
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
            "address" => {
                new_prod.store_address = Some(field.text().await.unwrap().to_owned());
            }
            field_name => {
                println!("unkonwn field name {}", field_name);
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
        Err(err) => {
            println!("err: {}", err.to_string());
            server_error()
        }
    }
}

pub async fn get_product_info(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id): Path<String>,
) -> Json<Value> {
    use crate::schema::products::dsl::*;

    let prod_id = match id.parse::<i32>() {
        Ok(pid) => pid,
        Err(_) => {
            return parameter_error();
        }
    };

    let conn = &mut pool.get().unwrap();

    let prod = products
        .select(Product::as_select())
        .filter(product_id.eq(prod_id))
        .first(conn)
        .unwrap();

    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "productId" : prod_id.to_string(),
            "productName" : prod.name.unwrap(),
            "description" : prod.description.unwrap(),
            "storeId" : prod.store_id.unwrap().to_string(),
            "detailImages" : prod.detail_images.unwrap(),
            "price" : prod.price.unwrap().to_string(),
            "sales" : prod.sales.unwrap().to_string(),
            "stock" : prod.stock.unwrap().to_string(),
            "address" : prod.store_address.unwrap(),
            "canbuy" : match prod.delete_product {
                Some(1) =>"0",
                _=>"1"
            }
        },
    }))
}

pub async fn get_recommend(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    use crate::schema::products::dsl::*;

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

    let results = products
        .offset((page_no * page_sz).into())
        .limit(page_sz.into())
        .filter(delete_product.ne(1))
        .order(sales.desc())
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
            "pageNo": "1", //页数
            "pageCount": "1", //总页数
            "total": result_vec.len().to_string(), //总记录数
            "list":result_vec,
        },
    }))
}

pub async fn remove_product(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    session: ReadableSession,
    Json(payload): Json<Value>,
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
    let prod_id = payload["productId"]
        .as_str()
        .unwrap()
        .to_string()
        .parse::<i32>()
        .unwrap();

    let conn = &mut pool.get().unwrap();

    match update(products)
        .set(delete_product.eq(1))
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
