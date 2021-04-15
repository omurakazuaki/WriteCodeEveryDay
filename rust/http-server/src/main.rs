use std::fs::File;
use std::io::{Error, Write, BufReader, BufWriter};
use std::io::prelude::*;
use std::collections::HashMap;

fn worker(stream: std::net::TcpStream) -> impl FnMut() -> Result<(), Error> {
    move || -> Result<(), Error> {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let req = read_request(reader)?;
        println!("{:?}", &req);
        let res = build_request(&req);
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
    body: Vec<u8>
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
        body: Vec::new()
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
        body.append(&mut buf[..n].to_vec());
    }
    if body.len() > 0 {
        request.body = body;
    }
    Ok(request)
}

struct Response {
    version: String,
    status: u16,
    message: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl Response {
    pub fn to_bytes(self) -> Vec<u8> {
        let first_line = format!("{} {} {}\r\n", self.version, self.status, self.message);
        let mut res = first_line.as_bytes().to_vec();
        for (key, val) in self.headers.iter() {
            res.append(&mut format!("{}: {}\r\n", key, val).as_bytes().to_vec());
        }
        res.append(&mut "\r\n".as_bytes().to_vec());
        res.append(&mut self.body.clone());
        res
    }
}

fn build_request(req: &Request) -> Response {
    match read_content(&req.target) {
        Ok(content) => {
            let mut headers = HashMap::new();
            headers.insert("Content-Length".to_string(), content.len().to_string());
            headers.insert("Content-Type".to_string(), "text/html".to_string()); // TODO: resolve content-type
            Response {
                version: "HTTP/1.1".to_string(),
                status: 200,
                message: "OK".to_string(),
                headers: headers,
                body: content
            }
        },
        Err(_) => { // TODO: error handling
            let content = read_content("/404.html").unwrap_or(Vec::new());
            let mut headers = HashMap::new();
            headers.insert("Content-Length".to_string(), content.len().to_string());
            headers.insert("Content-Type".to_string(), "text/html".to_string());
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

fn read_content(path: &str) -> std::io::Result<Vec<u8>> {
    let mut reader = BufReader::new(File::open(format!("./root{}", path))?);
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
    match std::net::TcpListener::bind("127.0.0.1:8080") {
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
