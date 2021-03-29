use super::schema::{posts, users};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize, Associations)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: Option<i32>,
}

#[derive(Queryable, Debug, Deserialize, Serialize, Associations)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}
