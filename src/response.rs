// "/" => stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),

use std::fmt::Display;

pub struct Response {
    pub header: Header,
    pub content_type: ContentType,
    pub content_lenght: usize,
    pub body: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\r\n{}\r\nContent-Length: {}\r\n\r\n{}\r\n",
            self.header, self.content_type, self.content_lenght, self.body,
        )
    }
}

pub struct Header {
    pub version: usize,
    pub code: StatusCode,
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/1.1 {}", self.code)
    }
}

pub enum StatusCode {
    Ok,
    Created,
    NotFound,
    InternalServerError,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::Created => write!(f, "201 Created"),
            Self::NotFound => write!(f, "404 NotFound"),
            Self::InternalServerError => write!(f, "500 InternalServerError"),
        }
    }
}

pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Content-Type: {}",
            match self {
                Self::TextPlain => "text/plain",
                Self::ApplicationOctetStream => "application/octet-stream",
            }
        )
    }
}
