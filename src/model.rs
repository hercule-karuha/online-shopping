use diesel::prelude::*;

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
        }
    }
}

#[derive(Insertable,AsChangeset, Queryable, Selectable)]
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
        }
    }
}