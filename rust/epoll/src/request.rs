use std::io::{Error, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub target: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>
}

impl Request {
    pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        let mut reader = BufReader::new(&bytes[..]);
        let first_line = &mut String::new();
        reader.read_line(first_line)?;
        let splits: Vec<&str> = first_line.split(" ").collect();
        let mut request = Self{
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
        // TODO: chunked
        let len = request.headers.get("Content-Length")
            .and_then(|v| Some(v.parse::<usize>().unwrap()))
            .unwrap_or(0);
        if len > 0 {
            request.body = Some(bytes.drain(bytes.len() - len..bytes.len()).collect());
        }
        Ok(request)
    }
}
