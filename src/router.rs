use serde::Serialize;

use serde_json::Value;
use std::collections::HashMap;

use crate::{method::HttpMethod, request::Request, response::Result};

pub type HandlerFn<T> = dyn Fn(Request) -> Result<T> + Send + Sync;

pub trait Handler<T>: Send + Sync + 'static
where
    T: Serialize,
{
    fn call(&self, req: Request) -> Result<T>;
}

impl<T, F> Handler<T> for F
where
    F: Fn(Request) -> Result<T> + Send + Sync + 'static,
    T: Serialize,
{
    fn call(&self, req: Request) -> Result<T> {
        (self)(req)
    }
}

pub trait RouteHandler: Send + Sync {
    fn path(&self) -> &str;
    fn method(&self) -> &HttpMethod;
    fn handle(&self, req: Request) -> Result<Value>;
}

pub struct Route<T>
where
    T: Serialize + 'static,
{
    pub path: String,
    pub method: HttpMethod,
    pub handler: Box<dyn Handler<T>>,
}

impl<T: Serialize + 'static> Route<T> {
    pub fn new<F>(path: String, method: HttpMethod, handler: F) -> Self
    where
        F: Handler<T> + 'static,
    {
        Self {
            path,
            method,
            handler: Box::new(handler),
        }
    }
}

impl<T> RouteHandler for Route<T>
where
    T: Serialize,
{
    fn path(&self) -> &str {
        &self.path
    }

    fn method(&self) -> &HttpMethod {
        &self.method
    }

    fn handle(&self, req: Request) -> Result<Value> {
        let res = self.handler.call(req);
        match res {
            Ok(v) => Ok(serde_json::json!(v)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Default)]
pub struct Router {
    routes: Vec<Box<dyn RouteHandler>>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route<T: Serialize>(&mut self, r: Route<T>) {
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
