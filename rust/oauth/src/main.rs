extern crate dotenv;

use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use actix_session::{CookieSession, Session};
use actix_web::{middleware::Logger, get, web, App, http::{header}, HttpResponse, HttpServer, Result, Error, error};
use tera::{Tera, Context};

#[derive(Deserialize, Debug)]
struct User {
    id: isize,
    login: String,
    avatar_url: String
}

#[get("/")]
async fn index(templates: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    match session.get::<String>("access_token").unwrap() {
        None => Ok(HttpResponse::TemporaryRedirect()
            .header(header::LOCATION, "/auth")
            .finish()),
        Some(access_token) => {
            let client = reqwest::Client::new();
            let res = client.get("https://api.github.com/user")
                .header(reqwest::header::USER_AGENT, "oauth-sample-app")
                .header(reqwest::header::AUTHORIZATION, format!("token {}", &access_token))
                .send()
                .await
                .unwrap();
            let body = res.text().await.unwrap();
            let user: User = serde_json::from_str(&body).unwrap();
            let mut ctx = Context::new();
            ctx.insert("name", &user.login);
            let view = templates.render("index.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(e))?;
            Ok(HttpResponse::Ok().content_type("text/html").body(view))
        }
    }
}

#[get("/auth")]
async fn auth(templates: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let client_id = env::var("CLIENT_ID").unwrap();
    //let redirect_uri = env::var("REDIRECT_URI").unwrap();
    let mut ctx = Context::new();
    ctx.insert("client_id", &client_id);
    //ctx.insert("redirect_uri", &redirect_uri);
    let view = templates.render("auth.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String
}

#[get("/callback")]
async fn callback(session: Session, params: web::Query<CallbackParams>) -> Result<HttpResponse, Error> {
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let client = reqwest::Client::new();
    let res = client.post("https://github.com/login/oauth/access_token")
        .header(reqwest::header::ACCEPT, "application/json")
        .form(&[("client_id", &client_id), ("client_secret", &client_secret), ("code", &params.code)])
        .send()
        .await
        .unwrap();
    let body = res.text().await.unwrap();
    let map = serde_json::from_str::<HashMap<&str, &str>>(&body).unwrap();
    let access_token: &str = map.get("access_token").unwrap();

    session.set("access_token", access_token)?;

    Ok(HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, "/")
        .finish()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .data(templates)
            .service(index)
            .service(auth)
            .service(callback)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
