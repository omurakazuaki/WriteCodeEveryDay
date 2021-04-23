use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

async fn process(stream: TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);

    let (reader, writer) = &mut (&stream, &stream);
    let mut buf = vec![0; 1024];
    reader.read(&mut buf).await?;
    println!("{}", String::from_utf8(buf).unwrap());
    let res = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/plan\r\n\r\nHello, World!";
    writer.write(res.as_bytes()).await?;
    Ok(())
}

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                process(stream).await.unwrap();
            });
        }
        Ok(())
    })
}
