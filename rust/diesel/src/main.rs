#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::NewPost;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool };
use dotenv::dotenv;

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = "sample.db";
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().max_size(4).build(manager).expect("Failed to create pool")
}

fn main() {
    dotenv().ok();

    let conn = establish_connection().get().unwrap();

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
