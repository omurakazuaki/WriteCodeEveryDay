#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

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
use num_bigint::{BigInt};
use num_traits::{Zero, One};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;

#[get("/")]
async fn index(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, templates: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    match session.get::<i32>("id").unwrap() {
        None => Ok(HttpResponse::TemporaryRedirect()
            .header(header::LOCATION, "/auth")
            .finish()),
        Some(id) => {
            match find_user_by_id(pool, id) {
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
                Ok(user) => {
                    let mut ctx = Context::new();
                    ctx.insert("name", &format!("{} {}", &user.given_name, &user.family_name));
                    let view = templates.render("index.html", &ctx)
                        .map_err(|e| error::ErrorInternalServerError(e))?;
                    Ok(HttpResponse::Ok().content_type("text/html").body(view))
                }
            }
        }
    }
}

#[get("/logout")]
async fn logout(session: Session) -> Result<HttpResponse, Error> {
    session.remove("id");
    Ok(HttpResponse::Found().header("Location", "/").finish())
}

#[get("/auth")]
async fn auth(templates: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let auth_url = env::var("AUTH_URL").unwrap();
    let client_id = env::var("CLIENT_ID").unwrap();
    let redirect_uri = env::var("REDIRECT_URI").unwrap();
    let state = Uuid::new_v4().to_hyphenated().to_string();
    let nonce = Uuid::new_v4().to_hyphenated().to_string();
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
async fn callback(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, session: Session, params: web::Form<CallbackParams>) -> Result<HttpResponse, Error> {
    match session.get::<String>("state").unwrap() {
        None => Ok(HttpResponse::Unauthorized().finish()),
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

                match verify_id_token(&token_res.id_token, &session).await {
                    Err(_) => Ok(HttpResponse::Unauthorized().finish()),
                    Ok(_) => {
                        let access_token = &token_res.access_token;
                        match create_user(pool, access_token).await {
                            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
                            Ok(user) => {
                                session.set("id", user.id)?;
                                Ok(HttpResponse::Found().header("Location", "/").finish())
                            }
                        }
                    }
                }
            } else {
                Ok(HttpResponse::BadRequest().finish())
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    sub: String,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    email: String,
    email_verified: bool,
    locale: String
}

fn find_user_by_id(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, id: i32) -> Result<models::User, ()> {
    let conn = pool.get().expect("database connection error");
    Ok(schema::users::dsl::users.find(id).first::<models::User>(&conn).expect("find user error"))
}

async fn create_user(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>, access_token: &str) -> Result<models::User, ()> {
    let user_info_url = env::var("USER_INFO_URL").unwrap();
    let client = reqwest::Client::new();
    let user_info = client.get(&user_info_url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", &access_token))
        .send()
        .await
        .unwrap()
        .json::<UserInfo>()
        .await
        .unwrap();
    let conn = pool.get().expect("database connection error");
    match schema::users::dsl::users.filter(schema::users::email.eq(&user_info.email)).first::<models::User>(&conn) {
        Err(_) => {
            let new_user = models::NewUser {
                email: &user_info.email,
                given_name: &user_info.given_name,
                family_name: &user_info.family_name
            };
            diesel::insert_into(schema::users::table).values(&new_user).execute(&conn).expect("insert error");
            Ok(schema::users::dsl::users.filter(schema::users::email.eq(&user_info.email)).first::<models::User>(&conn).expect("insert error"))
        },
        Ok(user) => Ok(user)
    }
}

#[derive(Deserialize, Debug)]
struct Jwk {
    alg: String,
    kid: String,
    n: String,
    e: String
}

#[derive(Deserialize, Debug)]
struct Jwks {
    keys: Vec<Jwk>
}

#[derive(Deserialize, Debug)]
struct IdTokenHeader {
    alg: String,
    kid: String
}

#[derive(Deserialize, Debug)]
struct IdTokenPayload {
    iss: String,
    sub: String,
    aud: String,
    exp: i64,
    iat: i64,
    nonce: String
}

async fn verify_id_token(id_token: &str, session: &Session) -> Result<(), ()> {
    let splits: Vec<&str> = id_token.split(".").collect();
    let header = serde_json::from_str::<IdTokenHeader>(&String::from_utf8(base64::decode(splits.get(0).unwrap()).unwrap()).unwrap()).unwrap();
    let payload = serde_json::from_str::<IdTokenPayload>(&String::from_utf8(base64::decode(splits.get(1).unwrap()).unwrap()).unwrap()).unwrap();
    let sign = base64::decode_config(splits.get(2).unwrap(), base64::URL_SAFE).unwrap();
    println!("id_token: {:?} {:?}", &header, &payload);

    let issuer = env::var("ISSUER").unwrap();
    assert!(issuer == payload.iss);

    let client_id = env::var("CLIENT_ID").unwrap();
    assert!(client_id == payload.aud);

    let now: DateTime<Utc> = Utc::now();
    assert!(now.timestamp() < payload.exp);

    let nonce = session.get::<String>("nonce").unwrap().unwrap();
    session.remove("nonce");
    assert!(nonce == payload.nonce);

    let jwk = fetch_jwk(&header.alg, &header.kid).await;
    let e = base64::decode_config(jwk.e, base64::URL_SAFE).unwrap();
    let n = base64::decode_config(jwk.n, base64::URL_SAFE).unwrap();
    let digest = decode_sign(sign, e, n);
    let expect = &digest[digest.len()-32..];
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}.{}", splits.get(0).unwrap(), splits.get(1).unwrap()));
    let mut actual = [0u8; 32];
    hasher.result(&mut actual);
    //println!("{:?} {:?}", actual, expect);
    assert!(actual == expect);

    Ok(())
}

async fn fetch_jwk(alg: &str, kid: &str) -> Jwk {
    let certs_url = env::var("CERTS_URL").unwrap();
    let client = reqwest::Client::new();
    let jwks = client.get(&certs_url)
        .send()
        .await
        .unwrap()
        .json::<Jwks>()
        .await
        .unwrap();
    jwks.keys.into_iter()
        .find(|jwk| jwk.alg == alg && jwk.kid == kid)
        .unwrap()
}

fn decode_sign(sign: Vec<u8>, e: Vec<u8>, n: Vec<u8>) -> Vec<u8> {
    let mut sign_big = sign.iter().fold(BigInt::zero(), |acc, n| acc * 256 + n);
    let mut e_big = e.iter().fold(BigInt::zero(), |acc, n| acc * 256 + n);
    let n_big = n.iter().fold(BigInt::zero(), |acc, n| acc * 256 + n);
    let m_big = mod_exp(&mut sign_big, &mut e_big, &n_big);
    m_big.to_signed_bytes_be()
}

fn mod_exp(base: &mut BigInt, exponent: &mut BigInt, modulus: &BigInt) -> BigInt {
    let one = BigInt::one();
    let zero = BigInt::zero();
    let mut result = BigInt::one();
    while &*exponent > &zero {
        if &*exponent & &one == one {
           result = (result * &*base) % modulus;
        }
        *base = (&*base * &*base) % modulus;
        *exponent = &*exponent >> 1usize;
    }
    result
}

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().max_size(4).build(manager).expect("Failed to create pool")
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
            .data(establish_connection())
            .service(index)
            .service(logout)
            .service(auth)
            .service(callback)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
