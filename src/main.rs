use std::net::TcpListener;
use std::io::Write;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                eprintln!("accepted new connection");
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
