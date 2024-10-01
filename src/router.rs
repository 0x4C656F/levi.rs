use serde_json::Value;
use std::collections::HashMap;
use serde_json::Serialize;

use crate::{errors::Exception, method::HttpMethod, request::Request};

pub trait<T: Serialize + Deserialize> Handler: Fn(Request) -> Result<T, Exception> + Send + Sync + 'static {}

impl<F> Handler for F where F: Fn(Request) -> Result<Value, Exception> + Send + Sync + 'static {}

pub trait RouteHandler: Send + Sync {
    fn path(&self) -> &str;
    fn method(&self) -> &HttpMethod;
    fn handle(&self, req: Request) -> Result<Value, Exception>;
}

impl<F: Handler> RouteHandler for Route<F> {
    fn path(&self) -> &str {
        &self.path
    }

    fn method(&self) -> &HttpMethod {
        &self.method
    }

    fn handle(&self, req: Request) -> Result<Value, Exception> {
        (self.handler_fn)(req)
    }
}

#[derive(Debug)]
pub struct Route<F: Handler> {
    pub path: String,
    pub method: HttpMethod,
    pub handler_fn: F,
}

#[derive(Default)]
pub struct Router {
    routes: Vec<Box<dyn RouteHandler>>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route<F: Handler>(&mut self, r: Route<F>) {
        self.routes.push(Box::new(r));
    }

    pub fn get_route(
        &self,
        raw_path: &str,
        method: &HttpMethod,
    ) -> Option<(&dyn RouteHandler, HashMap<String, String>)> {
        self.parse_path_from_request(raw_path, method)
    }

    fn parse_path_from_request(
        &self,
        path: &str,
        method: &HttpMethod,
    ) -> Option<(&dyn RouteHandler, HashMap<String, String>)> {
        for route in self.routes.iter().filter(|route| route.method() == method) {
            let mut params = HashMap::new();
            let req_path_split: Vec<&str> = path.trim_matches('/').split('/').collect();
            let route_path_split: Vec<&str> = route.path().trim_matches('/').split('/').collect();

            if req_path_split.len() != route_path_split.len() {
                continue;
            }

            let mut is_match = true;

            for (req_part, route_part) in req_path_split.iter().zip(route_path_split.iter()) {
                if let Some(field) = route_part.strip_prefix(':') {
                    params.insert(field.into(), (*req_part).to_string());
                } else if req_part != route_part {
                    is_match = false;
                    break;
                }
            }

            if is_match {
                return Some((route.as_ref(), params));
            }
        }
        None
    }
}
