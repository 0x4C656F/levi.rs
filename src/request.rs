use crate::body::parse_body;
use crate::method::{HttpMethod, RequestLine};
use serde_json::Value;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub headers: Vec<Header>,
    raw_body: Option<String>,
    pub method: HttpMethod,
    pub params: HashMap<String, String>,
    pub http_version: String,
    pub path: String,
}

impl Request {
    pub fn json(&self) -> Option<Value> {
        if let Some(raw) = &self.raw_body {
            let body = parse_body(raw.as_str()).ok();
            body
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct Header(pub String, pub String);

impl Header {
    pub fn from_header_str(s: &str) -> Option<Header> {
        let pair: Vec<&str> = s.split(':').map(|str| str.trim()).collect();
        if pair.len() != 2 {
            return None;
        }
        Some(Header(pair[0].to_owned(), pair[1].to_owned()))
    }
}

pub fn handle_request(stream: &mut TcpStream) -> Request {
    let mut buf = Vec::new();
    let mut reader = BufReader::new(stream);

    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .expect("Failed to read from stream");

        if line == "\r\n" || line == "\n" {
            break;
        }

        buf.extend_from_slice(line.as_bytes());
    }

    let req = String::from_utf8_lossy(&buf).into_owned();
    let entries: Vec<&str> = req.split("\r\n").collect();

    let req_line = RequestLine::try_from(entries[0]).expect("Invalid request line");

    let headers: Vec<Header> = entries
        .iter()
        .filter_map(|e| Header::from_header_str(e))
        .collect();

    let content_length = headers
        .iter()
        .find(|header| header.0 == "Content-Length")
        .map(|header| header.1.parse::<usize>().unwrap_or(0))
        .unwrap_or(0);

    let mut body = String::new();
    if content_length > 0 {
        reader
            .take(content_length as u64)
            .read_to_string(&mut body)
            .expect("Error reading body");
    }

    Request {
        raw_body: Some(body),
        headers,
        params: HashMap::new(),
        http_version: req_line.http_version.to_string(),
        method: req_line.method,
        path: req_line.path.to_string(),
    }
}
