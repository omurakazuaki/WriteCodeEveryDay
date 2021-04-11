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
use num_bigint::{BigInt};
use num_traits::{Zero, One};

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

                match verify_id_token(&token_res.id_token).await {
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

async fn verify_id_token(id_token: &str) -> std::io::Result<()> {
    let splits: Vec<&str> = id_token.split(".").collect();
    let header = serde_json::from_str::<IdTokenHeader>(&String::from_utf8(base64::decode(splits.get(0).unwrap()).unwrap()).unwrap()).unwrap();
    let payload = String::from_utf8(base64::decode(splits.get(1).unwrap()).unwrap()).unwrap();
    let sign = base64::decode_config(splits.get(2).unwrap(), base64::URL_SAFE).unwrap();
    println!("id_token: {:?} {} {:?}", &header, &payload, &sign);
    let certs_url = env::var("CERTS_URL").unwrap();
    let client = reqwest::Client::new();
    let jwks = client.get(&certs_url)
        .send()
        .await
        .unwrap()
        .json::<Jwks>()
        .await
        .unwrap();

    let jwk = jwks.keys.iter()
        .find(|jwk| jwk.alg == header.alg && jwk.kid == header.kid)
        .unwrap();
    let e_bytes = base64::decode_config(&jwk.e, base64::URL_SAFE).unwrap();
    let n_bytes = base64::decode_config(&jwk.n, base64::URL_SAFE).unwrap();
    //println!("jwk: {:?} {:?} {:?}", &jwk, &e_bytes, &n_bytes);

    let mut sign_big = sign.iter().fold(BigInt::zero(), |acc, n| acc * 256 + n);
    let mut e_big = e_bytes.iter().fold(BigInt::zero(), |acc, n| acc * 256 + n);
    let n_big = n_bytes.iter().fold(BigInt::zero(), |acc, n| acc * 256 + n);
    //println!("{} {} {}", &sign_big, &e_big, &n_big);
    let m_big = mod_exp(&mut sign_big, &mut e_big, &n_big);
    let digit =  &m_big.to_signed_bytes_be();
    let expect = &digit[digit.len()-32..];
    //println!("{:?}", expect);
    let mut hasher = Sha256::new();
    hasher.input_str(&format!("{}.{}", splits.get(0).unwrap(), splits.get(1).unwrap()));
    let mut actual = [0u8; 32];
    hasher.result(&mut actual);
    //println!("{:?}", hash);
    assert!(actual == expect);

    Ok(())
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
