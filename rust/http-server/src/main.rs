use std::fs::{self, File};
use std::path::{PathBuf};
use std::ffi::OsStr;
use std::io::{Error, Write, BufReader, BufWriter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time::Duration;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use tracing::{trace, error, instrument, Level};
use tracing_subscriber::FmtSubscriber;

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

fn get_error_page(code: usize) -> Vec<u8> {
    read_content(&PathBuf::from(&SETTINGS.root).join(&SETTINGS.error_pages.get(&code).unwrap_or(&String::new()))).unwrap_or(Vec::new())
}

static NOT_FOUND_CONTENT: Lazy<Vec<u8>> = Lazy::new(||get_error_page(404));
static SERVER_ERROR_CONTENT: Lazy<Vec<u8>> = Lazy::new(||get_error_page(500));

#[instrument]
fn worker(stream: TcpStream) -> impl FnMut() -> Result<(), Error> {
    move || -> Result<(), Error> {
        trace!("start {:?}", std::thread::current().id());
        stream.set_read_timeout(Some(Duration::new(0, 100_000_000)))
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
                            let res = build_response(&req);
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
            method: splits.get(0).unwrap().trim().to_string(),
            target: splits.get(1).unwrap().trim().to_string(),
            version: splits.get(2).unwrap().trim().to_string(),
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
        Ok(Some(request))
    }
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
fn build_response(req: &Request) -> Response {
    trace!("read {:?}", std::thread::current().id());
    let mut target_path = PathBuf::from(&SETTINGS.root).join(&req.target.trim_start_matches('/'));
    if target_path.is_dir() {
        target_path.push(PathBuf::from(&SETTINGS.index));
    }
    let connection = req.headers.get("Connection").unwrap_or(&"keep-alive".to_string()).clone();
    match fs::canonicalize(&target_path) {
        Ok(path) => {
            match read_content(&path) {
                Ok(content) => {
                    let mut headers = HashMap::new();
                    headers.insert("Content-Length".to_string(), content.len().to_string());
                    headers.insert("Content-Type".to_string(), resolve_content_type(&target_path));
                    headers.insert("Connection".to_string(), connection);
                    Response {
                        version: "HTTP/1.1".to_string(),
                        status: 200,
                        message: "OK".to_string(),
                        headers: headers,
                        body: Some(content)
                    }
                },
                Err(_) => {
                    let content = SERVER_ERROR_CONTENT.clone();
                    let mut headers = HashMap::new();
                    headers.insert("Content-Length".to_string(), content.len().to_string());
                    headers.insert("Content-Type".to_string(), "text/html".to_string());
                    headers.insert("Connection".to_string(), connection);
                    Response {
                        version: "HTTP/1.1".to_string(),
                        status: 500,
                        message: "Internal Server Error".to_string(),
                        headers: headers,
                        body: Some(content)
                    }
                }
            }
        },
        Err(_) => {
            let content = NOT_FOUND_CONTENT.clone();
            let mut headers = HashMap::new();
            headers.insert("Content-Length".to_string(), content.len().to_string());
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            headers.insert("Connection".to_string(), connection);
            Response {
                version: "HTTP/1.1".to_string(),
                status: 404,
                message: "Not Found".to_string(),
                headers: headers,
                body: Some(content)
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
