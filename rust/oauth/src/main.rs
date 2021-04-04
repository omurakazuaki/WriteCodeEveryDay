extern crate dotenv;

use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use actix_web::{get, web, App, HttpResponse, HttpServer, Result, Error, error};
use tera::{Tera, Context};

#[get("/")]
async fn index(templates: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let client_id = env::var("CLIENT_ID").unwrap();
    //let redirect_uri = env::var("REDIRECT_URI").unwrap();
    let mut ctx = Context::new();
    ctx.insert("client_id", &client_id);
    //ctx.insert("redirect_uri", &redirect_uri);
    let view = templates.render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String
}

#[get("/callback")]
async fn callback(params: web::Query<CallbackParams>) -> Result<HttpResponse, Error> {
    let mut body = HashMap::new();
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    body.insert("client_id", &client_id);
    body.insert("client_secret ", &client_secret);
    body.insert("code", &params.code);
    let client = reqwest::Client::new();
    let resp = client.post("https://github.com/login/oauth/access_token")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&body)
        .send()
        .await
        .unwrap();
    let body = resp.text().await.unwrap();
    // let json: serde_json::Value = serde_json::from_str(&body)?;
    // let obj = json.as_object().unwrap();
    // let access_token = obj.get("access_token").unwrap();

    Ok(HttpResponse::Ok().content_type("text/plane").body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(templates)
            .service(index)
            .service(callback)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
