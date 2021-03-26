#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::NewPost;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let database_url = "sample.db";
    let conn = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let new_post = NewPost { body: "title", title: "sample" };

    diesel::insert_into(schema::posts::table)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new post");

    let posts = schema::posts::dsl::posts
        .load::<models::Post>(&conn)
        .unwrap();

    println!("{:?}", posts);
}
