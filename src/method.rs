#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}
impl Default for HttpMethod {
    fn default() -> Self {
        Self::Get
    }
}

pub struct RequestLine<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub http_version: &'a str,
}

impl<'a> TryFrom<&'a str> for RequestLine<'a> {
    type Error = &'static str;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let header_split: Vec<&str> = v.split_whitespace().collect();

        if header_split.len() != 3 {
            return Err("Invalid request line format");
        }

        let method = match header_split[0] {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PATCH" => Ok(HttpMethod::Patch),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err("HTTP Method not found"),
        }?;

        Ok(RequestLine {
            method,
            path: header_split[1],
            http_version: header_split[2],
        })
    }
}
