use std::fs::{self, File};
use std::path::{PathBuf};
use std::io::{Error, Write, BufReader, BufWriter};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize};
use tracing::{info, trace, error, instrument, Level};
use tracing_subscriber::FmtSubscriber;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use chrono::offset::Utc;
use chrono::DateTime;
use flate2::Compression;
use flate2::bufread::GzEncoder;


#[derive(Debug, Deserialize)]
struct GzipSettings {
    enabled: bool,
    min_length: u64,
    types: Vec<String>
}

#[derive(Debug, Deserialize)]
struct Settings {
    listen: usize,
    server_name: String,
    root: String,
    index: String,
    types: HashMap<String, Vec<String>>,
    error_pages: HashMap<usize, String>,
    gzip: GzipSettings,
    buffer_size: usize
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
                    gzip: GzipSettings {
                        enabled: true,
                        min_length: 0,
                        types: Vec::new(),
                    },
                    buffer_size: 16384
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
        stream.set_nodelay(true)?;
        stream.set_read_timeout(Some(Duration::new(60, 0)))
            .expect("set_read_timeout call failed");
        let mut reader = BufReader::with_capacity(SETTINGS.buffer_size, &stream);
        let mut writer = BufWriter::with_capacity(SETTINGS.buffer_size, &stream);
        loop {
            match Request::read(&mut reader) {
                Err(_) => break,
                Ok(result) => {
                    let start = Instant::now();
                    match result {
                        None => continue,
                        Some(req) => {
                            let res = Response::build(&req, None);
                            let status = res.status.clone();
                            res.write(&mut writer)?;
                            let end = start.elapsed();
                            info!("{:?} {} {} {} {} {} ({:03}µs)",
                                std::thread::current().id(),
                                stream.peer_addr().unwrap(),
                                req.method,
                                req.target,
                                req.version,
                                status,
                                end.subsec_micros());
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

impl Request {
    #[instrument]
    fn read(reader: &mut BufReader<&TcpStream>) -> Result<Option<Self>, Error> {
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
                .unwrap_or(0);
            let mut read_num = 0;
            let mut body: Vec<u8> = Vec::new();
            while len > read_num {
                let mut buf = [0; 1024 * 8];
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
}

struct Response {
    version: String,
    status: usize,
    message: String,
    headers: HashMap<String, String>,
    reader: Option<Box<dyn Read>>
}

impl Response {
    pub fn new(status: usize, headers: HashMap<String, String>, reader: Option<Box<dyn Read>>) -> Self {
        Response {
            version: "HTTP/1.1".into(),
            status: status,
            message: MESSAGES.get(&status).unwrap_or(&String::new()).clone(),
            headers: headers,
            reader: reader
        }
    }

    #[instrument]
    pub fn build(req: &Request, mut status: Option<usize>) -> Self {
        trace!("read {:?}", std::thread::current().id());
        let mut target_path = match status {
            None => PathBuf::from(&SETTINGS.root).join(req.target.clone().trim_start_matches('/')),
            Some(sts) => PathBuf::from(&SETTINGS.error_pages.get(&sts).unwrap_or(&String::new()))
        };
        match fs::metadata(&target_path) {
            Err(_) => match status {
                None => Response::build(req, Some(404)),
                Some(sts) => Response::new(sts, HashMap::new(), None)
            },
            Ok(meta) => {
                if meta.is_dir() {
                    target_path.push(PathBuf::from(&SETTINGS.index));
                }
                match fs::metadata(&target_path) {
                    Err(_) => match status {
                        None => Response::build(req, Some(404)),
                        Some(sts) => Response::new(sts, HashMap::new(), None)
                    },
                    Ok(meta) => {
                        let mut headers = HashMap::new();
                        let content_type = Response::resolve_content_type(&target_path);
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
                        if accept_encodings.contains(&"gzip") && SETTINGS.gzip.enabled && SETTINGS.gzip.min_length <= meta.len() && SETTINGS.gzip.types.contains(&content_type) {
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
                        let reader: Option<Box<dyn Read>> = if status == Some(304) || req.method == "HEAD" {
                            None
                        } else if headers.get("Content-Encoding") == Some(&"gzip".into()) {
                            Some(Box::new(GzEncoder::new(BufReader::with_capacity(SETTINGS.buffer_size, File::open(target_path).unwrap()), Compression::fast())))
                        } else{
                            Some(Box::new(BufReader::with_capacity(SETTINGS.buffer_size, File::open(target_path).unwrap())))
                        };
                        Response::new(status.unwrap_or(200), headers, reader)
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

    pub fn write(self, writer: &mut BufWriter<&TcpStream>) -> Result<(), Error> {
        let mut res_as_string = format!("{} {} {}\r\n", &self.version, &self.status, &self.message);
        for (key, val) in self.headers.iter() {
            res_as_string.push_str(&format!("{}: {}\r\n", &key, &val));
        }
        res_as_string.push_str("\r\n");
        let mut res = res_as_string.as_bytes().to_vec();
        let is_chunked = self.headers.get("Transfer-Encoding") == Some(&"chunked".into());
        if is_chunked {
            writer.write(res_as_string.as_bytes())?;
        }
        if self.reader.is_some() {
            let mut reader = self.reader.unwrap();
            loop {
                let mut buf = [0; 1024 * 8];
                trace!("write1 {:?}", std::thread::current().id());
                let n = reader.read(&mut buf)?;
                if is_chunked {
                    trace!("write2 {:?} {}", std::thread::current().id(), n);
                    writer.write(format!("{:02x}\r\n", n).as_bytes())?;
                    writer.write(&mut buf[..n])?;
                    writer.write("\r\n".as_bytes())?;
                    writer.flush()?;
                    trace!("write3 {:?}", std::thread::current().id());
                } else {
                    res.append(&mut buf[..n].to_vec());
                }
                if n == 0 {
                    break;
                }
            }
        }
        if !is_chunked {
            writer.write(&res[..])?;
            writer.flush()?;
        }
        trace!("wrote {:?}", std::thread::current().id());
        Ok(())
    }
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
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
