use std::io::{Error, Read, Write};

fn worker(stream: std::net::TcpStream) -> impl FnMut() -> Result<(), Error> {
    move || -> Result<(), Error> {
        let mut reader = std::io::BufReader::new(&stream);
        let mut writer = std::io::BufWriter::new(&stream);
        let mut buf = [0; 1024];
        reader.read(&mut buf)?;
        println!("{}", std::str::from_utf8(&buf).unwrap());
        let res = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/plan\r\n\r\nHello, World!";
        writer.write(res.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
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
