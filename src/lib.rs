pub mod request;
pub mod response;

use request::Request;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut request = String::new();
                let mut buffer = BufReader::new(&stream);

                // TODO: Parse the input line by line, instead of copying it over and over.
                loop {
                    let mut line = String::new();
                    let size = buffer
                        .read_line(&mut line)
                        .expect("Error while reading from stream.");
                    request.push_str(&line);
                    if line == "\r\n" || size == 0 {
                        break;
                    }
                }
                let request = request.parse::<Request>().unwrap();

                eprintln!("{request:?}");

                let _ = match request.header.path.as_str() {
                    "/" => stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
                    _ => stream.write(b"HTTP/1.1 404 NotFound\r\n\r\n").unwrap(),
                };

                eprintln!("accepted new connection");
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
