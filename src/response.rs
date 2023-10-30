
// "/" => stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),

pub struct Response {
    pub header: Header,
    pub content_type: ContentType,
    pub content_lenght: usize,
}

pub struct Header {
    pub version: usize,
    pub code: StatusCode,
}

pub enum StatusCode {
    Ok,
    NotFound,
}

pub enum ContentType {
    TextPlain,
}
