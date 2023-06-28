use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub gender: Option<i32>,
    pub user_type: Option<i32>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub user_id: i32,
    pub user_name: Option<String>,
    pub gender: Option<i32>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

impl UpdateUser {
    pub fn new(user_id: i32) -> Self {
        UpdateUser {
            user_id,
            user_name: None,
            gender: None,
            phone: None,
            address: None,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDetail {
    pub user_id: i32,
    pub user_name: Option<String>,
    pub gender: Option<i32>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub user_type: Option<i32>,
}

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Store {
    pub store_id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub address: Option<String>,
}

impl Store {
    pub fn new(store_id: i32) -> Self {
        Store {
            store_id,
            user_id: None,
            name: None,
            address: None,
        }
    }
}

#[derive(AsChangeset, Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    pub product_id: i32,
    pub store_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub detail_images: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub sales: Option<i32>,
    pub store_address: Option<String>,
    pub delete_product: Option<i32>,
}

impl Product {
    pub fn new(store_id: Option<i32>) -> Self {
        Product {
            product_id: 1,
            store_id,
            name: None,
            description: None,
            detail_images: None,
            price: None,
            stock: None,
            sales: Some(0),
            store_address: None,
            delete_product: None,
        }
    }
}

#[derive(Insertable, AsChangeset, Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
pub struct ProductIns {
    pub store_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub detail_images: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub sales: Option<i32>,
    pub store_address: Option<String>,
    pub delete_product: Option<i32>,
}

impl ProductIns {
    pub fn new(store_id: Option<i32>) -> Self {
        ProductIns {
            store_id,
            name: None,
            description: None,
            detail_images: None,
            price: None,
            stock: None,
            sales: Some(0),
            store_address: None,
            delete_product: Some(0)
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductOrderInfo {
    pub price: Option<f64>,
    pub store_id: Option<i32>,
    pub store_address: Option<String>,
    pub name: Option<String>,
}
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::shopping_carts)]
pub struct CartInfo {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::orders)]
pub struct OrderInfo {
    pub order_id: i32,
    pub store_id: Option<i32>,
    pub product_id: Option<i32>,
    pub product_name: Option<String>,
    pub quantity: Option<i32>,
    pub total_price: Option<f64>,
    pub store_address: Option<String>,
    pub user_address: Option<String>,
    pub purchase_time: Option<NaiveDateTime>,
    pub user_phone: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(warnings)]
pub struct ProductInfo {
    pub productId: String,
    pub num: String,
}

#[derive(Debug, Deserialize)]
pub struct PurRequestData {
    pub address: String,
    pub phone: String,
    pub list: Vec<ProductInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CartRequestData {
    pub list: Vec<ProductInfo>,
}

#[derive(Debug, Deserialize)]
#[allow(warnings)]
pub struct PageInfo {
    pub pageSize: String,
    pub pageNo: String,
}

#[derive(Debug, Deserialize)]
#[allow(warnings)]
pub struct SearchInfo {
    pub keyword: String,
    pub storeId: String,
    pub pageSize: String,
    pub pageNo: String,
}
