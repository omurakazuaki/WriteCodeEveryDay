use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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

fn fetch_data() -> Vec<Todo> {
    vec![Todo {
            id: Some(1),
            title: "Tel".to_string(),
            description: "description".to_string(),
            status: Status::Open,
            deadline: "2021-03-14".to_string()
        },
        Todo {
            id: Some(2),
            title: "Programming".to_string(),
            description: "description".to_string(),
            status: Status::InProgress,
            deadline: "2021-03-15".to_string()
        }
    ]
}

#[get("/todos")]
async fn get_todos() -> impl Responder {
    HttpResponse::Ok().json(fetch_data())
}

#[get("/todo/{id}")]
async fn get_todo(web::Path(id): web::Path<u64>) -> impl Responder {
    match fetch_data().into_iter().find(|v| match &v.id {
        None => false,
        Some(vid) => *vid == id
    }) {
        None => HttpResponse::NotFound().json("not found"),
        Some(todo) => HttpResponse::Ok().json(todo)
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
