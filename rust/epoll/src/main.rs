use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::{File};
use std::io::{BufReader};
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use flate2::Compression;
use flate2::bufread::GzEncoder;
use tracing::{trace, info, Level};
use tracing_subscriber::FmtSubscriber;

use epoll::request::Request;
use epoll::response::Response;

const SERVER: Token = Token(0);

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    request: Option<Request>
}

#[derive(Debug)]
struct Server {
    poll: Poll,
    listener: TcpListener,
    clients: HashMap<Token, Client>,
    current_token: Token,
}

impl Server {
    fn new(ip: &str, port: u32) -> Self {
        let poll = Poll::new()
            .expect("poll new failed");

        let addr = format!("{}:{}", ip, port).parse().unwrap();
        let mut listener = TcpListener::bind(addr)
            .expect("socket bind failed");

        poll.registry()
            .register(&mut listener, SERVER, Interest::READABLE)
            .expect("poll registry register failed");
        Self {
            poll: poll,
            listener: listener,
            clients: HashMap::new(),
            current_token: Token(1)
        }
    }

    fn next_token(&mut self) -> Token {
        let token = self.current_token;
        self.current_token = Token(token.0 + 1);
        self.current_token
    }

    fn run(&mut self) {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None)
                .expect("event poll failed");

            for event in events.iter() {
                match event.token() {
                    SERVER => self.accept().unwrap(),
                    token => {
                        if event.is_readable() {
                            self.ready_read(token).unwrap()
                        }
                        if event.is_writable() {
                            self.ready_write(token).unwrap()
                        }
                    }
                }
            }
        }
    }

    fn accept(&mut self) -> Result<(), io::Error> {
        trace!("accept: start");
        loop {
            match self.listener.accept() {
                Ok((mut stream, adder)) => {
                    trace!("accept: {}", &adder);
                    stream.set_nodelay(true)?;
                    let new_token = self.next_token();
                    self.poll.registry().register(&mut stream, new_token, Interest::READABLE)?;
                    self.clients.insert(new_token, Client {stream: stream, request: None});
                },
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(err) => return Err(err)
            }
        }
    }

    fn ready_read(&mut self, token: Token) -> Result<(), io::Error> {
        match self.clients.get_mut(&token) {
            None => Ok(()),
            Some(client) => {
                let mut req = Vec::new();
                loop {
                    let mut buf = vec![0; 1024];
                    match client.stream.read(&mut buf) {
                        Ok(n) => {
                            if n == 0 {
                                break;
                            }
                            req.extend_from_slice(&mut buf[..n]);
                        },
                        Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => break,
                        Err(err) => return Err(err)
                    }
                }
                trace!("read: {}", req.len());
                if req.len() == 0 {
                    trace!("deregister");
                    self.poll.registry().deregister(&mut client.stream)?;
                    self.clients.remove(&token);
                } else {
                    client.request = Some(Request::from_bytes(&mut req).unwrap());
                    // TODO: chunked
                    self.poll.registry().reregister(&mut client.stream, token, Interest::WRITABLE)?;
                }
                Ok(())
            }
        }
    }

    fn ready_write(&mut self, token: Token) -> Result<(), io::Error> {
        match self.clients.get_mut(&token) {
            None => Ok(()),
            Some(client) => {
                match &client.request {
                    None => Ok(()),
                    Some(req) => {
                        let res = Response::from_request(&req, None);
                        let mut header_as_string = format!("{} {} {}\r\n", &res.version, &res.status, "message");
                        for (key, val) in res.headers.iter() {
                            header_as_string.push_str(&format!("{}: {}\r\n", &key, &val));
                        }
                        header_as_string.push_str("\r\n");
                        client.stream.write(header_as_string.as_bytes())?;
                        client.stream.flush()?;
                        if res.path.is_some() {
                            let is_chunked = res.headers.get("Transfer-Encoding") == Some(&"chunked".into());
                            let mut body = Vec::new();
                            let file_reader = BufReader::with_capacity(16384, File::open(res.path.unwrap()).unwrap());
                            let mut reader: Box<dyn Read> = if res.headers.get("Content-Encoding") == Some(&"gzip".into()) {
                                Box::new(GzEncoder::new(file_reader, Compression::new(6)))
                            } else {
                                Box::new(file_reader)
                            };
                            loop {
                                let mut buf = [0; 1024 * 8];
                                let n = reader.read(&mut buf)?;
                                if is_chunked {
                                    client.stream.write(format!("{:02x}\r\n", n).as_bytes())?;
                                    client.stream.write(&mut buf[..n])?;
                                    client.stream.write("\r\n".as_bytes())?;
                                    client.stream.flush()?;
                                } else {
                                    body.extend_from_slice(&mut buf[..n]);
                                }
                                if n == 0 {
                                    break;
                                }
                            }
                            if body.len() > 0 {
                                client.stream.write(&body[..])?;
                                client.stream.flush()?;
                            }
                        }
                        self.poll.registry().reregister(&mut client.stream, token, Interest::READABLE)?;

                        info!("{} {} {} {} {}",
                            client.stream.peer_addr().unwrap(),
                            req.method,
                            req.target,
                            req.version,
                            res.status);
                        Ok(())
                    }
                }
            }
        }
    }
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    Server::new("0.0.0.0", 8080).run();
}
