use std::fs::{self, File};
use std::path::{PathBuf};
use std::ffi::OsStr;
use std::io::{Error, Write, BufReader, BufWriter};
use std::io::prelude::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

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
                    server_name: "127.0.0.1".to_string(),
                    root: ".".to_string(),
                    index: "index.html".to_string(),
                    types: HashMap::new(),
                    error_pages: HashMap::new(),
                }
            },
        }
    }
}

static SETTINGS: Lazy<Settings> = Lazy::new(||Settings::load());

fn get_error_page(code: usize) -> Option<Vec<u8>>{
    match read_content(&PathBuf::from(&SETTINGS.root).join(&SETTINGS.error_pages.get(&code).unwrap_or(&String::new()))) {
        Err(_) => None,
        Ok(content) => Some(content)
    }
}

static NOT_FOUND_CONTENT: Lazy<Option<Vec<u8>>> = Lazy::new(||get_error_page(404));
static SERVER_ERROR_CONTENT: Lazy<Option<Vec<u8>>> = Lazy::new(||get_error_page(500));

fn worker(stream: std::net::TcpStream) -> impl FnMut() -> Result<(), Error> {
    move || -> Result<(), Error> {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let req = read_request(reader)?;
        println!("{:?}", &req);
        let res = build_response(&req);
        writer.write(&res.to_bytes()[..])?;
        writer.flush()?;
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

fn read_request(mut reader: BufReader<&std::net::TcpStream>) -> Result<Request, Error> {
    let first_line = &mut String::new();
    reader.read_line(first_line)?;
    let splits: Vec<&str> = first_line.split(" ").collect();
    let mut request = Request{
        method: splits.get(0).unwrap().trim().to_string(),
        target: splits.get(1).unwrap().trim().to_string(),
        version: splits.get(2).unwrap().trim().to_string(),
        headers: HashMap::new(),
        body: None
    };
    request.target.remove(0);
    loop {
        let header_line = &mut String::new();
        reader.read_line(header_line)?;
        if header_line == "\r\n" {
            break;
        }
        let key_value: Vec<&str> = header_line.split(":").collect();
        let key = key_value.get(0).unwrap().trim().to_string();
        let val = key_value.get(1).unwrap().trim().to_string();
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
    Ok(request)
}

struct Response {
    version: String,
    status: u16,
    message: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>
}

impl Response {
    pub fn to_bytes(self) -> Vec<u8> {
        let first_line = format!("{} {} {}\r\n", self.version, self.status, self.message);
        let mut res = first_line.as_bytes().to_vec();
        for (key, val) in self.headers.iter() {
            res.extend_from_slice(&mut format!("{}: {}\r\n", key, val).as_bytes());
        }
        res.extend_from_slice(&mut "\r\n".as_bytes());
        res.append(&mut self.body.clone().unwrap_or(Vec::new()));
        res
    }
}

fn build_response(req: &Request) -> Response {
    let mut target_path = PathBuf::from(&SETTINGS.root).join(&req.target);
    if target_path.is_dir() {
        target_path.push(PathBuf::from(&SETTINGS.index));
    }
    let blank = String::new();
    // TODO: */*
    let accept_types: Vec<&str> = req.headers.get("Accept").unwrap_or(&blank).split(",").collect();
    match fs::canonicalize(&target_path) {
        Ok(path) => {
            match read_content(&path) {
                Ok(content) => {
                    let mut headers = HashMap::new();
                    headers.insert("Content-Length".to_string(), content.len().to_string());
                    headers.insert("Content-Type".to_string(), resolve_content_type(&target_path)); // TODO: resolve content-type
                    Response {
                        version: "HTTP/1.1".to_string(),
                        status: 200,
                        message: "OK".to_string(),
                        headers: headers,
                        body: Some(content)
                    }
                },
                Err(_) => {
                    let content = if accept_types.contains(&"text/html") { SERVER_ERROR_CONTENT.clone() } else { None };
                    let mut headers = HashMap::new();
                    if content.is_some() {
                        headers.insert("Content-Length".to_string(), SERVER_ERROR_CONTENT.clone().unwrap().len().to_string());
                        headers.insert("Content-Type".to_string(), "text/html".to_string());
                    }
                    Response {
                        version: "HTTP/1.1".to_string(),
                        status: 500,
                        message: "Internal Server Error".to_string(),
                        headers: headers,
                        body: content
                    }
                }
            }
        },
        Err(_) => {
            let content = if accept_types.contains(&"text/html") { NOT_FOUND_CONTENT.clone() } else { None };
            let mut headers = HashMap::new();
            if content.is_some() {
                headers.insert("Content-Length".to_string(), NOT_FOUND_CONTENT.clone().unwrap().len().to_string());
                headers.insert("Content-Type".to_string(), "text/html".to_string());
            }
            Response {
                version: "HTTP/1.1".to_string(),
                status: 404,
                message: "Not Found".to_string(),
                headers: headers,
                body: content
            }
        }
    }
}

fn resolve_content_type(path: &PathBuf) -> String {
    let ext = path.extension().unwrap_or(OsStr::new("")).to_str().unwrap().to_string();
    match SETTINGS.types.iter().find(|(_, v)|v.contains(&ext)) {
        None => String::new(),
        Some((types, _)) => types.clone()
    }
}

fn read_content(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut content: Vec<u8> = Vec::new();
    loop {
        let mut buf = [0; 1024 * 1024];
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        content.append(&mut buf[..n].to_vec());
    }
    Ok(content)
}

fn main() {
    match std::net::TcpListener::bind(format!("{}:{}", SETTINGS.server_name, SETTINGS.listen)) {
        Err(e) => {
            eprintln!("socket bind error: {}", e);
        },
        Ok(listener) => {
            for streams in listener.incoming() {
                match streams {
                    Err(e) => {
                        eprintln!("accept error: {}", e);
                    },
                    Ok(stream) => {
                        std::thread::spawn(worker(stream));
                    }
                }

            }
        }
    }
}
