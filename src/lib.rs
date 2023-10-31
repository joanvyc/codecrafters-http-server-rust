pub mod request;
pub mod response;

use request::Request;
use tokio::{
    io::{AsyncBufReadExt, BufReader, AsyncWriteExt, Result},
    net::{TcpListener, TcpStream}, task::JoinSet, select,
};

use crate::response::{Response, StatusCode, ContentType};

async fn process_request(mut stream: TcpStream) -> Result<(TcpStream, Request)> {

    let mut reader = BufReader::new(&mut stream);

    let mut line = String::new();
    let mut request = String::new();

    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => {
                // Connection closed.
                break;
            },
            Ok(_) => {
                request.push_str(&line);
                if &line == "\r\n" { break; }
                line.clear();
            },
            Err(e) => {
                eprintln!("Error reading from the stream: {e}");
            },
        }
    }

    Ok((
        stream,
        request.parse::<Request>().expect("a parsable request"),
    ))
}

async fn process_response(mut stream: TcpStream, request: Request) -> Result<()> {

    let response = match request.start_line.path.as_str() {
        "/" => Response {
            header: response::Header { version: 1, code: StatusCode::Ok },
            content_type: ContentType::TextPlain,
            content_lenght: 0,
            body: "".to_string(),
        },
        "/user-agent" => {
            if let Some(body) = request.header.get("User-Agent") {
                Response {
                    header: response::Header { version: 1, code: StatusCode::Ok },
                    content_type: ContentType::TextPlain,
                    content_lenght: body.len(),
                    body: body.to_owned(),
                }
            } else {
                Response {
                    header: response::Header { version: 1, code: StatusCode::NotFound },
                    content_type: ContentType::TextPlain,
                    content_lenght: 0,
                    body: "".to_string(),
                }
            }
        },
        s if s.starts_with("/echo/") => { 
            let body = s.strip_prefix("/echo/").unwrap();
            Response { 
                header: response::Header { version: 1, code: StatusCode::Ok },
                content_type: ContentType::TextPlain,
                content_lenght: body.len(),
                body: body.to_string(),
            }
        }
        _ => Response {
            header: response::Header { version: 1, code: StatusCode::NotFound },
            content_type: ContentType::TextPlain,
            content_lenght: 0,
            body: "".to_string(),
        },
    };

    stream.write(response.to_string().as_bytes()).await.unwrap();

    Ok(())
}

pub async fn run_server() {

    let mut requests = JoinSet::new();
    let mut responses = JoinSet::new();

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        select! {
            Ok((stream, _)) = listener.accept() => {
                requests.spawn(process_request(stream));
            }

            Some(Ok(Ok((stream, request)))) = requests.join_next() => {
                responses.spawn(process_response(stream, request));
            }

            Some(Ok(Ok(()))) = responses.join_next() => {}
        }
    }

}
