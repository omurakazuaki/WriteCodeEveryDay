#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod db;

use diesel::prelude::*;
use self::db::establish_connection;

use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};

#[get("/users")]
async fn get_users() -> impl Responder {
    match establish_connection().get() {
        Err(_) => HttpResponse::InternalServerError().json("error"),
        Ok(conn) => match schema::users::dsl::users.load::<models::User>(&conn) {
            Err(_) => HttpResponse::InternalServerError().json("error"),
            Ok(users) => match serde_json::to_string(&users) {
                Err(_) => HttpResponse::InternalServerError().json("error"),
                Ok(posts_as_json) => HttpResponse::Ok().json(&posts_as_json)
            }
        }
    }
}

#[get("/users/{id}")]
async fn get_user(web::Path(id): web::Path<i32>) -> impl Responder {
    match establish_connection().get() {
        Err(_) => HttpResponse::InternalServerError().json("error"),
        Ok(conn) => match schema::users::dsl::users.find(id).first::<models::User>(&conn) {
            Err(_) => HttpResponse::InternalServerError().json("error"),
            Ok(user) => match serde_json::to_string(&user) {
                Err(_) => HttpResponse::InternalServerError().json("error"),
                Ok(user_as_json) => HttpResponse::Ok().json(&user_as_json)
            }
        }
    }
}

#[post("/users")]
async fn post_users(req_body: String) -> impl Responder {
    match serde_json::from_str::<models::NewUser>(&req_body) {
        Err(_) => HttpResponse::BadRequest().json("parse error"),
        Ok(new_user) => match establish_connection().get() {
            Err(_) => HttpResponse::InternalServerError().json("error"),
            Ok(conn) => match diesel::insert_into(schema::users::table).values(&new_user).execute(&conn) {
                Err(_) => HttpResponse::InternalServerError().json("error"),
                Ok(count) => HttpResponse::Ok().json(count)
            }
        }
    }
}

#[get("/posts")]
async fn get_posts() -> impl Responder {
    match establish_connection().get() {
        Err(_) => HttpResponse::InternalServerError().json("error"),
        Ok(conn) => match schema::posts::dsl::posts.load::<models::Post>(&conn) {
            Err(_) => HttpResponse::InternalServerError().json("error"),
            Ok(posts) => match serde_json::to_string(&posts) {
                Err(_) => HttpResponse::InternalServerError().json("error"),
                Ok(posts_as_json) => HttpResponse::Ok().json(&posts_as_json)
            }
        }
    }
}

#[get("/posts/{id}")]
async fn get_post(web::Path(id): web::Path<i32>) -> impl Responder {
    match establish_connection().get() {
        Err(_) => HttpResponse::InternalServerError().json("error"),
        Ok(conn) => match schema::posts::dsl::posts.find(id).first::<models::Post>(&conn) {
            Err(_) => HttpResponse::InternalServerError().json("error"),
            Ok(post) => match serde_json::to_string(&post) {
                Err(_) => HttpResponse::InternalServerError().json("error"),
                Ok(post_as_json) => HttpResponse::Ok().json(&post_as_json)
            }
        }
    }
}

#[post("/posts")]
async fn post_posts(req_body: String) -> impl Responder {
    match serde_json::from_str::<models::NewPost>(&req_body) {
        Err(_) => HttpResponse::BadRequest().json("parse error"),
        Ok(new_post) => match establish_connection().get() {
            Err(_) => HttpResponse::InternalServerError().json("error"),
            Ok(conn) => match diesel::insert_into(schema::posts::table).values(&new_post).execute(&conn) {
                Err(_) => HttpResponse::InternalServerError().json("error"),
                Ok(count) => HttpResponse::Ok().json(count)
            }
        }
    }
}

#[delete("/posts/{id}")]
async fn delete_post(web::Path(id): web::Path<i32>) -> impl Responder {
    match establish_connection().get() {
        Err(_) => HttpResponse::InternalServerError().json("error"),
        Ok(conn) => match diesel::delete(schema::posts::dsl::posts.find(id)).execute(&conn) {
            Err(_) => HttpResponse::InternalServerError().json("error"),
            Ok(count) => HttpResponse::Ok().json(&count)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_posts)
            .service(get_post)
            .service(post_posts)
            .service(delete_post)
            .service(get_user)
            .service(get_users)
            .service(post_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
