use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

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

fn fetch_data() -> io::Result<Vec<Todo>> {
    match fs::read_to_string("./target/data.json") {
        Ok(content) => Ok(serde_json::from_str(&content)?),
        Err(e) => Err(e),
    }
}

fn save_data(todos: &Vec<Todo>) -> io::Result<()> {
    let todos_as_json = serde_json::to_string(&todos)?;
    fs::write("./target/data.json", todos_as_json)?;
    Ok(())
}

#[get("/todos")]
async fn get_todos() -> impl Responder {
    match fetch_data() {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("error")
    }
}

#[get("/todos/{id}")]
async fn get_todo(web::Path(id): web::Path<u64>) -> impl Responder {
    match fetch_data() {
        Ok(data) => {
            match data.iter().find(|v| match v.id {
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

#[post("/todos")]
async fn post_todo(req_body: String) -> impl Responder {
    match serde_json::from_str::<Todo>(&req_body) {
        Ok(mut todo) => {
            match fetch_data() {
                Ok(mut todos) => {
                    todos.sort_by(|a, b| b.id.partial_cmp(&a.id).unwrap());
                    todo.id = Some(match todos.get(0) {
                        None => 1,
                        Some(t) => t.id.unwrap() + 1,
                    });
                    todos.push(todo);
                    match save_data(&todos) {
                        Ok(_) => HttpResponse::Ok().json(todos.get(todos.len() -1).unwrap()),
                        Err(_) => HttpResponse::InternalServerError().json("error")
                    }
                },
                Err(_) => HttpResponse::InternalServerError().json("error")
            }
        },
        Err(_) => HttpResponse::BadRequest().json("parse error")
    }
}

#[delete("/todos/{id}")]
async fn delete_todo(web::Path(id): web::Path<u64>) -> impl Responder {
    match fetch_data() {
        Ok(mut data) => {
            match data.iter().position(|v| v.id.unwrap() == id) {
                None => HttpResponse::NotFound().json("not found"),
                Some(index) => {
                    data.remove(index);
                    match save_data(&data) {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(_) => HttpResponse::InternalServerError().json("error")
                    }
                }
            }

        },
        Err(_) => HttpResponse::InternalServerError().json("error")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_todos)
            .service(get_todo)
            .service(post_todo)
            .service(delete_todo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
