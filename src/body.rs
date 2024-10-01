use serde_json::{Result, Value};

pub fn parse_body(body_string: &str) -> Result<Value> {
    let trimmed_body = body_string.trim_end_matches('\0');

    serde_json::from_str(trimmed_body)
}
