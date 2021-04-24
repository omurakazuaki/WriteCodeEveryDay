use tokio::io::{self, BufReader, AsyncRead, AsyncReadExt, AsyncBufReadExt, AsyncWriteExt};
use tokio::fs::{File};
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use async_compression::tokio::bufread::GzipEncoder;

use asyncsample::request::Request;
use asyncsample::response::Response;


async fn process(mut stream: TcpStream) -> io::Result<()> {
    stream.set_nodelay(true)?;
    loop {
        match read(&mut stream).await {
            Err(e) => return Err(e),
            Ok(result) => {
                match result {
                    None => {},
                    Some(request) => {
                        writer(&mut stream, request).await?;
                    }
                }
            }
        };
    }
}

async fn read(reader: &mut TcpStream) -> io::Result<Option<Request>> {
    let mut reader = BufReader::new(reader);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).await?;
    if first_line.len() == 0 {
        // keep alive
        return Ok(None);
    }
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
        reader.read_line(header_line).await?;
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
    let mut read_num = 0;
    let mut body: Vec<u8> = Vec::new();
    while len > read_num {
        let mut buf = [0; 1024 * 8];
        let n = reader.read(&mut buf).await?;
        read_num += n;
        body.extend_from_slice(&mut buf[..n]);
            }
    Ok(Some(request))
}

async fn writer(writer: &mut TcpStream, request: Request) -> io::Result<()> {
    let res = Response::from_request(&request, None).await;
    let mut header_as_string = format!("{} {} {}\r\n", &res.version, &res.status, "message");
    for (key, val) in res.headers.iter() {
        header_as_string.push_str(&format!("{}: {}\r\n", &key, &val));
    }
    header_as_string.push_str("\r\n");
    writer.write(header_as_string.as_bytes()).await?;
    writer.flush().await?;
    if res.path.is_some() {
        let is_chunked = res.headers.get("Transfer-Encoding") == Some(&"chunked".into());
        let mut body = Vec::new();
        let mut file_reader = BufReader::new(File::open(res.path.unwrap()).await.unwrap());
        let is_gzip = res.headers.get("Content-Encoding") == Some(&"gzip".into());
        if is_gzip {
            let mut gzip_reader = GzipEncoder::new(file_reader);
            loop {
                let mut buf = [0; 1024 * 8];
                let n = gzip_reader.read(&mut buf).await?;
                if is_chunked {
                    writer.write(format!("{:02x}\r\n", n).as_bytes()).await?;
                    writer.write(&mut buf[..n]).await?;
                    writer.write("\r\n".as_bytes()).await?;
                    writer.flush().await?;
                } else {
                    body.extend_from_slice(&mut buf[..n]);
                }
                if n == 0 {
                    break;
                }
            }
        } else {
            loop {
                let mut buf = [0; 1024 * 8];
                let n = file_reader.read(&mut buf).await?;
                if is_chunked {
                    writer.write(format!("{:02x}\r\n", n).as_bytes()).await?;
                    writer.write(&mut buf[..n]).await?;
                    writer.write("\r\n".as_bytes()).await?;
                    writer.flush().await?;
                } else {
                    body.extend_from_slice(&mut buf[..n]);
                }
                if n == 0 {
                    break;
                }
            }
        }
        if body.len() > 0 {
            writer.write(&body[..]).await?;
            writer.flush().await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async {
            process(stream).await.unwrap();
        });
    }
}
