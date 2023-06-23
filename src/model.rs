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

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Store {
    pub store_id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct {
    pub store_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub detail_images: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

impl NewProduct {
    pub fn new(store_id: Option<i32>) -> Self {
        NewProduct {
            store_id,
            name: None,
            description: None,
            detail_images: None,
            price: None,
            stock: None,
        }
    }
}
