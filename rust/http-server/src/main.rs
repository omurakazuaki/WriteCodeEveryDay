use std::fs::{self, File};
use std::path::{PathBuf};
use std::io::{Error, Write, BufReader, BufWriter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time::Duration;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use tracing::{trace, error, instrument, Level};
use tracing_subscriber::FmtSubscriber;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use chrono::offset::Utc;
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    listen: usize,
    server_name: String,
    root: String,
    index: String,
    types: HashMap<String, Vec<String>>,
    error_pages: HashMap<usize, String>
}

impl Settings {
    pub fn load() -> Self {
        match fs::read_to_string("./settings.yml") {
            Ok(str) => {
                serde_yaml::from_str(&str).unwrap()
            },
            Err(_) => {
                Settings {
                    listen: 80,
                    server_name: "127.0.0.1".into(),
                    root: ".".into(),
                    index: "index.html".into(),
                    types: HashMap::new(),
                    error_pages: HashMap::new(),
                }
            },
        }
    }
}

static SETTINGS: Lazy<Settings> = Lazy::new(||Settings::load());

static MESSAGES: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    let mut messages = HashMap::new();
    messages.insert(200, "OK".into());
    messages.insert(304, "Not Modified".into());
    messages.insert(404, "Not Found".into());
    messages.insert(500, "Internal Server Error".into());
    messages
});

#[instrument]
fn worker(stream: TcpStream) -> impl FnMut() -> Result<(), Error> {
    move || -> Result<(), Error> {
        trace!("start {:?}", std::thread::current().id());
        stream.set_read_timeout(Some(Duration::new(60, 0)))
            .expect("set_read_timeout call failed");
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        loop {
            match read_request(&mut reader) {
                Err(_) => break,
                Ok(result) => {
                    match result {
                        None => continue,
                        Some(req) => {
                            let res = build_response(&req, None);
                            writer.write(&res.to_bytes()[..])?;
                            writer.flush()?;
                            trace!("end {:?}", std::thread::current().id());
                            if let Some(val) = req.headers.get("Connection") {
                                if val == "close" {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        stream.shutdown(Shutdown::Both)?;
        trace!("close {:?}", std::thread::current().id());
        Ok(())
    }
}

#[derive(Debug)]
struct Request {
    method: String,
    target: String,
    version: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>
}

#[instrument]
fn read_request(reader: &mut BufReader<&TcpStream>) -> Result<Option<Request>, Error> {
    let first_line = &mut String::new();
    reader.read_line(first_line)?;
    if first_line.len() == 0 {
        // keep alive
        Ok(None)
    } else {
        let splits: Vec<&str> = first_line.split(" ").collect();
        let mut request = Request{
            method: splits.get(0).unwrap().trim().into(),
            target: splits.get(1).unwrap().trim().into(),
            version: splits.get(2).unwrap().trim().into(),
            headers: HashMap::new(),
            body: None
        };
        loop {
            let header_line = &mut String::new();
            reader.read_line(header_line)?;
            if header_line == "\r\n" {
                break;
            }
            let key_value: Vec<&str> = header_line.split(":").collect();
            let key = key_value.get(0).unwrap().trim().into();
            let val = key_value.get(1).unwrap().trim().into();
            request.headers.insert(key, val);
        }
        let len = request.headers.get("Content-Length")
            .and_then(|v| Some(v.parse::<usize>().unwrap()))
            .or(Some(0)).unwrap();
        let mut read_num = 0;
        let mut body: Vec<u8> = Vec::new();
        while len > read_num {
            let mut buf = [0; 1024];
            let n = reader.read(&mut buf)?;
            read_num += n;
            body.extend_from_slice(&mut buf[..n]);
        }
        if body.len() > 0 {
            request.body = Some(body);
        }
        Ok(Some(request))
    }
}

struct Response {
    version: String,
    status: usize,
    message: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>
}

impl Response {
    pub fn new(status: usize, headers: HashMap<String, String>, body: Option<Vec<u8>>) -> Response {
        Response {
            version: "HTTP/1.1".into(),
            status: status,
            message: MESSAGES.get(&status).unwrap_or(&String::new()).clone(),
            headers: headers,
            body: body
        }
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut res_as_string = format!("{} {} {}\r\n", self.version, self.status, self.message);
        for (key, val) in self.headers.iter() {
            res_as_string.push_str(&format!("{}: {}\r\n", key, val));
        }
        res_as_string.push_str("\r\n");
        let mut res = res_as_string.as_bytes().to_vec();
        if self.body.is_some() {
            res.append(&mut self.body.clone().unwrap());
        }
        res
    }
}

#[instrument]
fn build_response(req: &Request, mut status: Option<usize>) -> Response {
    trace!("read {:?}", std::thread::current().id());
    let mut target_path = PathBuf::from(&SETTINGS.root);
    let path = match status {
        None => req.target.clone(),
        Some(sts) => SETTINGS.error_pages.get(&sts).unwrap_or(&String::new()).clone()
    };
    target_path.push(&path.trim_start_matches('/'));
    match fs::metadata(&target_path) {
        Err(_) => match status {
            None => build_response(req, Some(404)),
            Some(sts) => Response::new(sts, HashMap::new(), None)
        },
        Ok(meta) => {
            if meta.is_dir() {
                target_path.push(PathBuf::from(&SETTINGS.index));
            }
            match fs::metadata(&target_path) {
                Err(_) => match status {
                    None => build_response(req, Some(404)),
                    Some(sts) => Response::new(sts, HashMap::new(), None)
                },
                Ok(meta) => {
                    let mut headers = HashMap::new();
                    headers.insert("Content-Length".into(), meta.len().to_string());
                    headers.insert("Content-Type".into(), resolve_content_type(&target_path));
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
                        let cache_control = req.headers.get("Cache-Control").unwrap_or(&String::new()).clone();
                        let if_none_match = req.headers.get("If-None-Match");
                        if cache_control != "no-store" {
                            if if_none_match == headers.get("ETag") {
                                status = Some(304);
                            }
                        }
                    }
                    let body = if status == Some(304) || req.method == "HEAD" {
                        Ok(None)
                    } else {
                        read_content(&target_path).map(|content|Some(content))
                    };
                    match body {
                        Err(_) => build_response(req, Some(500)),
                        Ok(body) => {
                            let status = status.unwrap_or(200);
                            Response::new(status, headers, body)
                        }
                    }
                }
            }
        }
    }
}

fn resolve_content_type(path: &PathBuf) -> String {
    let ext = match path.extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or("")
    };
    match SETTINGS.types.iter().find(|(_, v)|v.contains(&ext.into())) {
        None => String::new(),
        Some((types, _)) => types.clone()
    }
}

fn read_content(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut content: Vec<u8> = Vec::new();
    loop {
        let mut buf = [0; 1024];
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        content.append(&mut buf[..n].to_vec());
    }
    Ok(content)
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let listener = TcpListener::bind(format!("{}:{}", SETTINGS.server_name, SETTINGS.listen))
        .expect("socket bind error");
    trace!("bind");
    for incoming in listener.incoming() {
        trace!("incoming");
        match incoming {
            Err(e) => {
                error!("accept error: {}", e);
            },
            Ok(stream) => {
                std::thread::spawn(worker(stream));
            }
        }
        trace!("next");
    }
}
