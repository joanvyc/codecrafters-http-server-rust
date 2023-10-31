pub mod request;
pub mod response;

use std::env::args;
use std::path::Path;
use std::{io::ErrorKind, str::from_utf8};

use request::Request;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Result, AsyncReadExt},
    net::{TcpListener, TcpStream},
    select,
    task::JoinSet, fs::File,
};

use crate::response::{ContentType, Response, StatusCode};

async fn process_request(mut stream: TcpStream) -> Result<(TcpStream, Request)> {
    let mut reader = BufReader::new(&mut stream);

    let mut line = String::new();
    let mut request = String::new();

    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => {
                // Connection closed.
                break;
            }
            Ok(_) => {
                request.push_str(&line);
                if &line == "\r\n" {
                    break;
                }
                line.clear();
            }
            Err(e) => {
                eprintln!("Error reading from the stream: {e}");
            }
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
            header: response::Header {
                version: 1,
                code: StatusCode::Ok,
            },
            content_type: ContentType::TextPlain,
            content_lenght: 0,
            body: "".to_string(),
        },
        "/user-agent" => {
            if let Some(body) = request.header.get("User-Agent") {
                Response {
                    header: response::Header {
                        version: 1,
                        code: StatusCode::Ok,
                    },
                    content_type: ContentType::TextPlain,
                    content_lenght: body.len(),
                    body: body.to_owned(),
                }
            } else {
                Response {
                    header: response::Header {
                        version: 1,
                        code: StatusCode::NotFound,
                    },
                    content_type: ContentType::TextPlain,
                    content_lenght: 0,
                    body: "".to_string(),
                }
            }
        }
        s if s.starts_with("/echo/") => {
            let body = s.strip_prefix("/echo/").unwrap();
            Response {
                header: response::Header {
                    version: 1,
                    code: StatusCode::Ok,
                },
                content_type: ContentType::TextPlain,
                content_lenght: body.len(),
                body: body.to_string(),
            }
        }
        s if s.starts_with("/files/") => {

            let base_dir = if Some("--directory".to_string()) == args().nth(1) {
                args().nth(2).expect("missing directory file")
            } else {
                "./".to_string()
            };

            let file_name = s.strip_prefix("/files/").unwrap();
            let file_name = Path::new(&base_dir).join(file_name);

            match File::open(file_name).await {
                Ok(mut file) => {

                    let size = file.metadata().await?.len();

                    let mut file_content = Vec::with_capacity(size as usize);
                    let _len = file.read_to_end(&mut file_content).await;

                    Response {
                        header: response::Header {
                            version: 1,
                            code: StatusCode::Ok,
                        },
                        content_type: ContentType::ApplicationOctetStream,
                        content_lenght: file_content.len(),
                        body: from_utf8(&file_content).unwrap().to_string(),
                    }
                },

                Err(e) if e.kind() == ErrorKind::NotFound => Response {
                    header: response::Header {
                        version: 1,
                        code: StatusCode::NotFound,
                    },
                    content_type: ContentType::TextPlain,
                    content_lenght: "Not found".len(),
                    body: "Not found".to_string(),
                },

                Err(_) => Response {
                    header: response::Header {
                        version: 1,
                        code: StatusCode::Ok,
                    },
                    content_type: ContentType::TextPlain,
                    content_lenght: "Internal server error".len(),
                    body: "Internal server error".to_string(),
                },
            }
        }
        _ => Response {
            header: response::Header {
                version: 1,
                code: StatusCode::NotFound,
            },
            content_type: ContentType::TextPlain,
            content_lenght: 0,
            body: "".to_string(),
        },
    };

    let response_serialized = response.to_string();
    let size = stream.write(response_serialized.as_bytes()).await.unwrap();
    if size != response_serialized.as_bytes().len() {
        eprintln!("Bytes sent does not match bytes to sent.");
    }

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
