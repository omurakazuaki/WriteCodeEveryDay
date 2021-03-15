use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{ Result };

#[derive(Debug, Deserialize, Serialize)]
enum Status {
    Open,
    InProgress,
    Close
}

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: Option<u64>,
    title: String,
    description: String,
    status: Status,
    deadline: String
}

fn fetch_data() -> Result<Vec<Todo>> {
    match fs::read_to_string("./target/data.json") {
        Ok(content) => Ok(serde_json::from_str(&content)?),
        Err(e) => Err(e),
    }
}

#[get("/todos")]
async fn get_todos() -> impl Responder {
    match fetch_data() {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("error")
    }
}

#[get("/todo/{id}")]
async fn get_todo(web::Path(id): web::Path<u64>) -> impl Responder {
    match fetch_data() {
        Ok(data) => {
            match data.into_iter().find(|v| match v.id {
                    None => false,
                    Some(vid) => vid == id
                }) {
                    None => HttpResponse::NotFound().json("not found"),
                    Some(todo) => HttpResponse::Ok().json(todo)
                }
            }
        Err(_) => HttpResponse::InternalServerError().json("error")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_todos)
            .service(get_todo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
