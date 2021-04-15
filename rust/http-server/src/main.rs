use std::io::{Error, Write};
use std::io::prelude::*;
use std::collections::HashMap;

fn worker(stream: std::net::TcpStream) -> impl FnMut() -> Result<(), Error> {
    move || -> Result<(), Error> {
        let reader = std::io::BufReader::new(&stream);
        let mut writer = std::io::BufWriter::new(&stream);
        let req = read_request(reader)?;
        println!("{:?}", &req);
        let res = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/plan\r\n\r\nHello, World!";
        writer.write(res.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug)]
struct Request {
    method: String,
    target: String,
    version: String,
    header: HashMap<String, String>,
    body: Option<String>
}

fn read_request(mut reader: std::io::BufReader<&std::net::TcpStream>) -> Result<Request, Error> {
    let first_line = &mut String::new();
    reader.read_line(first_line)?;
    let splits: Vec<&str> = first_line.split(" ").collect();
    let mut request = Request{
        method: splits.get(0).unwrap().trim().to_string(),
        target: splits.get(1).unwrap().trim().to_string(),
        version: splits.get(2).unwrap().trim().to_string(),
        header: HashMap::new(),
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
        request.header.insert(key, val);
    }
    let len = request.header.get("Content-Length")
        .and_then(|v| Some(v.parse::<usize>().unwrap()))
        .or(Some(0)).unwrap();
    let mut read_num = 0;
    let mut body_as_bytes: Vec<u8> = Vec::new();
    while len > read_num {
        let mut buf = [0; 1024];
        let n = reader.read(&mut buf)?;
        read_num += n;
        body_as_bytes.append(&mut buf[..n].to_vec());
    }
    if body_as_bytes.len() > 0 {
        request.body = Some (std::str::from_utf8(&body_as_bytes).unwrap().to_string())
    }
    Ok(request)
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
