use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{crlf, not_line_ending},
    sequence::{terminated, tuple},
};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Request {
    pub start_line: Header,
    pub header: HashMap<String, String>,
}

impl FromStr for Request {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parse_header_line = terminated(
            tuple((
                terminated(take_until(":"), tag(": ")),
                not_line_ending::<_, ()>,
            )),
            crlf,
        );

        let mut header = HashMap::new();

        let request = s;
        let (request, start_line) = parse_start_line(request).unwrap();
        let mut request = request;
        loop {
            println!("Parsing request: \n{request}");
            if request == "\r\n" || request.is_empty() {
                break;
            }
            let (rem_request, (key, value)) = parse_header_line(request).unwrap();
            header.insert(key.to_string(), value.to_string());
            request = rem_request;
        }

        Ok(Request { start_line, header })
    }
}

#[derive(Debug)]
pub struct Header {
    pub method: Method,
    pub path: String,
    pub version: HTTPVersion,
}

fn parse_start_line(value: &str) -> nom::IResult<&str, Header> {
    let mut parse = tuple((
        terminated(alt((tag("GET"), tag("POST"))), tag(" ")),
        terminated(take_until(" "), tag(" ")),
        terminated(not_line_ending, crlf),
    ));

    let (value, (method, path, version)) = parse(value)?;
    Ok((
        value,
        Header {
            method: method.parse().unwrap(),
            path: path.to_string(),
            version: version.parse().unwrap(),
        },
    ))
}

#[derive(Debug)]
pub enum Method {
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
pub enum HTTPVersion {
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
