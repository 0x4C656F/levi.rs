use std::{fmt::Display, io::Write, net::TcpStream};

use serde_json::Value;

use crate::{request::Header, Exception, HTTP_VERSION};
pub fn handle_response(
    s: &mut TcpStream,
    res: crate::response::Result<Value>,
) -> std::io::Result<usize> {
    match res {
        Ok(body) => {
            let mut res = ResponseBuilder::new();
            res.add_response_line(ResponseLine::default());
            res.add_header(Header(
                "Content-Type".to_string(),
                crate::content_type::ContentType::ApplicationJson.to_string(),
            ));
            res.add_body(body.to_string());
            res.send(s)
        }
        Err(e) => {
            let mut res = ResponseBuilder::new();
            res.add_response_line(ResponseLine {
                http_version: HTTP_VERSION.into(),
                status_code: e.status_code,
                status_text: e.status_text,
            });
            res.add_header(Header(
                "Content-Type".to_string(),
                crate::content_type::ContentType::ApplicationJson.to_string(),
            ));
            res.send(s)
        }
    }
}

pub type Result<T> = std::result::Result<T, Exception>;

pub struct ResponseLine {
    http_version: String,
    status_code: u32,
    status_text: String,
}

impl Display for ResponseLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            self.http_version, self.status_code, self.status_text
        )
    }
}

impl Default for ResponseLine {
    fn default() -> Self {
        Self {
            status_code: 200,
            status_text: "OK".into(),
            http_version: HTTP_VERSION.into(),
        }
    }
}

#[derive(Debug)]
pub struct ResponseBuilder {
    response_string: String,
}
impl ResponseBuilder {
    fn add_header(&mut self, h: Header) -> &mut Self {
        self.response_string
            .push_str(&format!("{}: {}\n", h.0, h.1));
        self
    }
    fn add_body(&mut self, body: String) -> &mut Self {
        self.add_header(Header("Content-Length".to_string(), body.len().to_string()));
        self.response_string.push_str("\r\n");
        self.response_string.push_str(&body);
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

    fn send(self, tcp: &mut TcpStream) -> std::io::Result<usize> {
        tcp.write(self.response_string.as_bytes())
    }
}
