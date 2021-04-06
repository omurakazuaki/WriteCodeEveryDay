extern crate open;

use std::env;
use std::thread;
use dotenv::dotenv;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct PostDeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64
}

#[derive(Deserialize, Debug)]
struct PostAccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Deserialize, Debug)]
struct User {
    id: isize,
    login: String,
    avatar_url: String
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: String,
    error_description: String,
    error_uri: String,
}

async fn run() {
    dotenv().ok();
    let client_id = env::var("CLIENT_ID").unwrap();

    let client = reqwest::Client::new();
    let post_device_code_url = format!("https://github.com/login/device/code?client_id={}&scope=user:email", client_id);
    let post_device_code_res: PostDeviceCodeResponse = client.post(&post_device_code_url)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{}", post_device_code_res.user_code);

    open::that(post_device_code_res.verification_uri).unwrap();

    loop {
        thread::sleep(Duration::from_secs(post_device_code_res.interval));
        let post_access_token_url = format!("https://github.com/login/oauth/access_token?client_id={}&device_code={}&grant_type=urn:ietf:params:oauth:grant-type:device_code",
            client_id, post_device_code_res.device_code);
        let res_as_text = client.post(&post_access_token_url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        match serde_json::from_str::<PostAccessTokenResponse>(&res_as_text) {
            Err(_) => {
                let error: ErrorResponse = serde_json::from_str(&res_as_text).unwrap();
                if error.error == "authorization_pending" {
                    continue;
                } else {
                    panic!();
                }
            },
            Ok(post_access_token_res) => {
                let user: User = client.get("https://api.github.com/user")
                .header(reqwest::header::USER_AGENT, "oauth-sample-app")
                .header(reqwest::header::AUTHORIZATION, format!("token {}", post_access_token_res.access_token))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                println!("{:?}", user);
                break;
            }
        }
    }

}

#[tokio::main]
async fn main() {
    run().await
}
