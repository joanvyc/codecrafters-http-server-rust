use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{crlf, not_line_ending},
    sequence::{delimited, terminated, tuple},
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub header: Header,
    pub host: String,
    pub user_agent: String,
}

impl FromStr for Request {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parse_host = delimited(tag("Host: "), not_line_ending::<_, ()>, crlf);
        let mut parse_user_agent = delimited(tag("User-Agent: "), not_line_ending::<_, ()>, crlf);

        let (value, header) = parse_header(s).unwrap();
        let (value, host) = parse_host(value).unwrap();
        let (_, user_agent) = parse_user_agent(value).unwrap();

        Ok(Request {
            header,
            host: host.to_string(),
            user_agent: user_agent.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct Header {
    pub method: Method,
    pub path: String,
    pub version: HTTPVersion,
}

fn parse_header(value: &str) -> nom::IResult<&str, Header> {
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
