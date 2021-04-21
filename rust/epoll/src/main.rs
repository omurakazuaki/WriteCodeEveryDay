use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use tracing::{trace, instrument, Level};
use tracing_subscriber::FmtSubscriber;

const SERVER: Token = Token(0);

#[derive(Debug)]
struct Client {
    stream: TcpStream
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

    #[instrument]
    fn run(&mut self) {
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None)
                .expect("event poll failed");

            for event in events.iter() {
                match event.token() {
                    SERVER => self.accept().unwrap(),
                    token => self.ready(token).unwrap()
                }
            }
        }
    }

    #[instrument]
    fn accept(&mut self) -> Result<(), io::Error> {
        trace!("accept: start");
        loop {
            match self.listener.accept() {
                Ok((mut stream, _)) => {
                    trace!("accept");
                    stream.set_nodelay(true)
                        .expect("set_nodelay call failed");
                    let new_token = self.next_token();
                    self.poll.registry().register(&mut stream, new_token, Interest::READABLE)
                        .expect("client registry failed");
                    self.clients.insert(new_token, Client {stream: stream});
                },
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(err) => return Err(err)
            }
        }
    }

    #[instrument]
    fn ready(&mut self, token: Token) -> Result<(), io::Error> {
        match self.clients.get_mut(&token) {
            None => Ok(()),
            Some(client) => {
                let mut buf = vec![0; 1024];
                match client.stream.read(&mut buf) {
                    Ok(n) => {
                        if n == 0 {
                            trace!("deregister");
                            self.poll.registry().deregister(&mut client.stream)?;
                        } else {
                            trace!("read: {} {}", n, String::from_utf8(buf).unwrap());
                            let res = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/plan\r\n\r\nHello, World!";
                            client.stream.write(res.as_bytes())?;
                            client.stream.flush()?;
                            trace!("write");
                        }
                        Ok(())
                    },
                    Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => Ok(()),
                    Err(err) => Err(err)
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
