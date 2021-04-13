use super::schema::{users};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Debug, Deserialize, Serialize, Associations)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub given_name: &'a str,
    pub family_name: &'a str,
}
