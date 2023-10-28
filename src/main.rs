use std::net::TcpListener;
use std::io::{Write, Read};
use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    sequence::{delimited, terminated, tuple},
    character::complete::{not_line_ending, crlf},
};

use std::str::FromStr;

#[derive(Debug)]
struct Request {
    header: Header,
    host: String,
    user_agent: String,
}

impl FromStr for Request {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parse_host = delimited(tag("Host: "), not_line_ending::<_, ()>, crlf);
        let mut parse_user_agent = delimited(tag("User-Agent: "), not_line_ending::<_, ()>, crlf);

        let (value, header) = parse_header(s).unwrap();
        let (value, host) = parse_host(value).unwrap();
        let (_, user_agent) = parse_user_agent(value).unwrap();

        Ok(Request{
            header,
            host: host.to_string(),
            user_agent: user_agent.to_string(),
        })
    }
}

#[derive(Debug)]
struct Header {
    method: Method,
    path: String,
    version: HTTPVersion,
}

fn parse_header(value: &str) -> nom::IResult<&str, Header> {
    let mut parse = tuple((
        terminated(alt((tag("GET"), tag("POST"))), tag(" ")),
        terminated(take_until(" "), tag(" ")),
        terminated(not_line_ending, crlf),
    ));

    let (value, (method, path, version)) = parse(value)?;
    Ok((value, Header{
        method: method.parse().unwrap(),
        path: path.to_string(),
        version: version.parse().unwrap(),
    }))
}

#[derive(Debug)]
enum Method {
    Get,
}

impl FromStr for Method {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Method::*;
        match s {
            "GET" => Ok(Get),
            _ => bail!("Unknown method: {s}"),
        }
    }
}

#[derive(Debug)]
enum HTTPVersion {
    Http1_1,
}

impl FromStr for HTTPVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use HTTPVersion::*;
        match s {
            "HTTP/1.1" => Ok(Http1_1),
            _ => bail!("Unknown http version: {s}"),
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut request = String::new();
                let _ = stream.read_to_string(&mut request).unwrap();
                let request = request.parse::<Request>().unwrap();

                eprintln!("{request:?}");

                let _ = match request.header.path.as_str() {
                    "/" => stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
                    _   => stream.write(b"HTTP/1.1 404 NotFound\r\n\r\n").unwrap(),
                };

                eprintln!("accepted new connection");
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
