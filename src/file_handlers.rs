use axum::{
    body::{Bytes, Full},
    extract::{Multipart, Path, State},
    response::{Json, Response},
};

use axum_sessions::extractors::ReadableSession;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{insert_into, update};
use hyper::{header::CONTENT_TYPE, StatusCode};
use serde_json::{json, Value};

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use uuid::Uuid;

use crate::model::*;

use crate::error_return::*;

pub async fn upload_image(session: ReadableSession, mut multipart: Multipart) -> Json<Value> {
    match session.get::<i32>("id") {
        Some(_) => 1,
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

    let image_field = match multipart.next_field().await {
        Ok(cf) => cf,
        Err(_) => {
            return parameter_error();
        }
    };

    let image_field = image_field.unwrap();
    let image_type = image_field.content_type().unwrap().to_string();
    let image = image_field.bytes().await.unwrap();
    let image_name = Uuid::new_v4().to_string();
    let mut path = "images/".to_string() + image_name.as_str() + ".";

    if let Some(index) = image_type.rfind('/') {
        let extension = &image_type[index + 1..];
        path.push_str(extension);
        let mut file = match File::create(path.clone()).await {
            Ok(fe) => fe,
            Err(error) => {
                println!("{}", error);
                return server_error();
            }
        };
        file.write_all(&image).await.expect("write error");
    }
    Json(json!({
        "code": 200,
        "msg": "请求成功",
        "data": {
            "path" : path
        },
    }))
}

pub async fn get_image(Path(image_path): Path<String>) -> Response<Full<Bytes>> {
    let mut f = match File::open(image_path).await {
        Ok(fe) => fe,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::from("not found"))
                .unwrap();
        }
    };

    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer).await {
        Ok(_) => Response::builder()
            .header(CONTENT_TYPE, "image")
            .body(Full::from(buffer))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Full::from("read error"))
            .unwrap(),
    }
}
