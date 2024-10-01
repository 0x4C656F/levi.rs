use std::{io::Write, net::TcpStream};

use crate::{
    method::HttpMethod,
    request::{Header, Request},
    HTTP_VERSION,
};
pub fn handle_response(s: &mut TcpStream) -> ResponseBuilder {
    let mut res = ResponseBuilder::new();
    res.add_response_line(ResponseLine::default());
    res.add_header(Header("Content-Type".to_string(), "text/html".to_string()));
    res.add_body("<html><body>Hello, World!</body></html>".to_string());
    let _ = s.write_all(res.response_string.as_bytes());
    res
}

#[derive(Debug)]
pub struct ResponseBuilder {
    response_string: String,
}

pub struct ResponseLine {
    http_version: String,
    status_code: u32,
    status: String,
}
impl ToString for ResponseLine {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}\n",
            self.http_version, self.status_code, self.status
        )
    }
}
impl Default for ResponseLine {
    fn default() -> Self {
        Self {
            status_code: 200,
            status: "OK".into(),
            http_version: HTTP_VERSION.into(),
        }
    }
}

impl ResponseBuilder {
    fn add_header(&mut self, h: Header) -> &mut Self {
        self.response_string
            .push_str(&format!("{}: {}\n", h.0, h.1));
        self
    }
    fn add_body(&mut self, body: String) -> &mut Self {
        self.add_header(Header("Content-Length".to_string(), body.len().to_string()));
        self.response_string.push_str("\r\n"); // End headers section
        self.response_string.push_str(&body); // Append body
        self
    }
    fn add_response_line(&mut self, line: ResponseLine) -> &mut Self {
        self.response_string.push_str(&line.to_string());
        self
    }
    fn new() -> Self {
        Self {
            response_string: String::new(),
        }
    }
}
