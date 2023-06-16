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
