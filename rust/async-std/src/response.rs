use std::collections::HashMap;
use tokio::fs;
use std::path::{PathBuf};
use chrono::offset::Utc;
use chrono::DateTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use async_recursion::async_recursion;
use crate::request::Request;

pub struct Response {
    pub version: String,
    pub status: usize,
    pub headers: HashMap<String, String>,
    pub path: Option<PathBuf>
}

fn resolve_content_type(path: &PathBuf) -> String {
    let ext = match path.extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or("")
    };
    let types = [
      ("text/html", vec!["html", "htm", "shtml"]),
      ("text/css", vec!["css"]),
      ("image/gif", vec!["gif"]),
      ("image/png", vec!["png"]),
      ("image/jpeg", vec!["jpeg", "jpg"]),
      ("image/svg+xml", vec!["svg", "svgz"]),
      ("image/x-icon", vec!["ico"]),
      ("application/javascript", vec!["js"]),
      ("application/json", vec!["json", "map"]),
    ];
    match types.iter().find(|(_, v)|v.contains(&ext.into())) {
        None => String::new(),
        Some((types, _)) => types.to_string()
    }
}

impl Response {
    pub fn new(status: usize, headers: HashMap<String, String>, path: Option<PathBuf>) -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status: status,
            headers: headers,
            path: path
        }
    }

    #[async_recursion]
    pub async fn from_request(req: &Request, mut status: Option<usize>) -> Self {
        let mut target_path = match status {
            None => PathBuf::from("../epoll/root").join(req.target.clone().trim_start_matches('/')),
            Some(sts) => PathBuf::from(format!("../epoll/errors/{}.html", sts))
        };
        match fs::metadata(&target_path).await {
            Err(_) => match status {
                None => Response::from_request(req, Some(404)).await,
                Some(sts) => {
                    let mut headers = HashMap::new();
                    headers.insert("Content-Length".into(), "0".into());
                    Response::new(sts, headers, None)
                }
            },
            Ok(meta) => {
                if meta.is_dir() {
                    target_path.push(PathBuf::from("index.html"));
                }
                match fs::metadata(&target_path).await {
                    Err(_) => match status {
                        None => Response::from_request(req, Some(404)).await,
                        Some(sts) => {
                            let mut headers = HashMap::new();
                            headers.insert("Content-Length".into(), "0".into());
                            Response::new(sts, headers, None)
                        }
                    },
                    Ok(meta) => {
                        let mut headers = HashMap::new();
                        let content_type = resolve_content_type(&target_path);
                        headers.insert("Content-Type".into(), content_type.clone());
                        let connection = req.headers.get("Connection").unwrap_or(&"keep-alive".into()).clone();
                        headers.insert("Connection".into(), connection);
                        let date = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
                        headers.insert("Date".into(), date);
                        let target_path_as_str = target_path.to_str();
                        let modified = meta.modified();
                        if status.is_none() && target_path_as_str.is_some() && modified.is_ok() {
                            let datetime: DateTime<Utc> = modified.unwrap().into();
                            let modified_as_str = datetime.format("%a, %d %b %Y %T GMT").to_string();
                            let mut hasher = Sha256::new();
                            hasher.input_str(&target_path_as_str.unwrap());
                            let e_tag = format!("{}-{:x}-{:x}", &hasher.result_str()[..8], meta.len(), datetime.timestamp());
                            headers.insert("Last-Modified".into(), modified_as_str);
                            headers.insert("ETag".into(), e_tag);
                        }
                        let accept_encodings: Vec<&str> = match req.headers.get("Accept-Encoding") {
                            None => Vec::new(),
                            Some(values) => values.split(",")
                                .map(|v|v.trim())
                                .collect()
                        };
                        if accept_encodings.contains(&"gzip") && 16384 <= meta.len() {
                            headers.insert("Content-Encoding".into(), "gzip".into());
                            headers.insert("Transfer-Encoding".into(), "chunked".into());
                        } else {
                            headers.insert("Accept-Ranges".into(),"bytes".into());
                            headers.insert("Content-Length".into(), meta.len().to_string());
                        }
                        let cache_control = req.headers.get("Cache-Control");
                        let if_none_match = req.headers.get("If-None-Match");
                        if if_none_match.is_some() && cache_control != Some(&"no-store".into()) {
                            if if_none_match == headers.get("ETag") {
                                status = Some(304);
                            }
                        }
                        let path: Option<PathBuf> = if status == Some(304) || req.method == "HEAD" {
                            None
                        } else {
                            Some(target_path)
                        };
                        Response::new(status.unwrap_or(200), headers, path)
                    }
                }
            }
        }
    }
}
