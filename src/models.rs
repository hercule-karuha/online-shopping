use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub user_id: i32,
    pub account: String,
    pub password: String,
    pub gender: Option<String>,
    pub user_type: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}
