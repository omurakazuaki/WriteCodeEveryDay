extern crate dotenv;

use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use actix_session::{CookieSession, Session};
use actix_web::{middleware::Logger, get, post, web, App, http::{header}, HttpResponse, HttpServer, Result, Error, error};
use tera::{Tera, Context};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use url::form_urlencoded;
use base64;

#[derive(Deserialize, Debug)]
struct User {
    sub: String,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    email: String,
    email_verified: bool,
    locale: String
}

#[get("/")]
async fn index(templates: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    match session.get::<String>("access_token").unwrap() {
        None => Ok(HttpResponse::TemporaryRedirect()
            .header(header::LOCATION, "/auth")
            .finish()),
        Some(access_token) => {
            let user_info_url = env::var("USER_INFO_URL").unwrap();
            let client = reqwest::Client::new();
            let user = client.get(&user_info_url)
                .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", &access_token))
                .send()
                .await
                .unwrap()
                .json::<User>()
                .await
                .unwrap();
            let mut ctx = Context::new();
            ctx.insert("name", &user.name);
            let view = templates.render("index.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(e))?;
            Ok(HttpResponse::Ok().content_type("text/html").body(view))
        }
    }
}

#[get("/auth")]
async fn auth(templates: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let auth_url = env::var("AUTH_URL").unwrap();
    let client_id = env::var("CLIENT_ID").unwrap();
    let redirect_uri = env::var("REDIRECT_URI").unwrap();
    let mut hasher = Sha256::new();
    hasher.input_str("// TODO: random state");
    let state = hasher.result_str();
    let mut hasher = Sha256::new();
    hasher.input_str("// TODO: random nonce");
    let nonce = hasher.result_str();
    let mut ctx = Context::new();
    let query_parameters = form_urlencoded::Serializer::new(String::new())
        .append_pair("response_type", "id_token code")
        .append_pair("client_id", &client_id)
        .append_pair("scope", "openid email profile")
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("state", &state)
        .append_pair("nonce", &nonce)
        .append_pair("response_mode", "form_post")
        .finish();
    ctx.insert("auth_url", &auth_url);
    ctx.insert("query_parameters", &query_parameters);
    let view = templates.render("auth.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    session.set("state", &state)?;
    session.set("nonce", &nonce)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[derive(Deserialize, Debug)]
struct CallbackParams {
    code: String,
    state: String,
    scope: String,
    id_token: String
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    scope: String,
    token_type: String,
    id_token: String,
}

#[post("/callback")]
async fn callback(session: Session, params: web::Form<CallbackParams>) -> Result<HttpResponse, Error> {
    match session.get::<String>("state").unwrap() {
        None => Ok(HttpResponse::BadRequest().finish()),
        Some(state) => {
            if state == params.state {
                let token_url = env::var("TOKEN_URL").unwrap();
                let client_id = env::var("CLIENT_ID").unwrap();
                let client_secret = env::var("CLIENT_SECRET").unwrap();
                let redirect_uri = env::var("REDIRECT_URI").unwrap();
                let client = reqwest::Client::new();
                let token_res = client.post(&token_url)
                    .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .form(&[
                        ("code", params.code.clone()),
                        ("client_id", client_id),
                        ("client_secret", client_secret),
                        ("redirect_uri", redirect_uri),
                        ("grant_type", "authorization_code".to_string())
                        ])
                    .send()
                    .await
                    .unwrap()
                    .json::<TokenResponse>()
                    .await
                    .unwrap();

                match verify_id_token(&token_res.id_token) {
                    Err(_) => Ok(HttpResponse::BadRequest().finish()),
                    Ok(_) => {
                        let access_token = &token_res.access_token;

                        session.set("access_token", access_token)?;

                        Ok(HttpResponse::Found().header("Location", "/").finish())
                    }
                }
            } else {
                Ok(HttpResponse::BadRequest().finish())
            }
        }
    }

}

fn verify_id_token(id_token: &str) -> std::io::Result<()> {
    let vec: Vec<&str> = id_token.split(".").collect();
    let header = String::from_utf8(base64::decode(vec.get(0).unwrap()).unwrap()).unwrap();
    let payload = String::from_utf8(base64::decode(vec.get(1).unwrap()).unwrap()).unwrap();
    let sign = vec.get(2).unwrap();

    println!("id_token: {} {} {:?}", header, payload, sign);
    // TODO: verify id_token
    Ok(())
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
