use crate::errors::Exception;
use crate::request;
use crate::request::Request;
use crate::response;
use crate::router::Router;
use crate::HttpMethod;
use serde_json::Value;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
pub struct Levi {
    pub socket: TcpListener,
    pub port: u32,
    pub router: Router,
}
impl Levi {
    pub fn new(port: u32) -> Result<Self, std::io::Error> {
        let socket = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        Ok(Self {
            socket,
            port,
            router: Router::new(),
        })
    }
    pub fn listen(self) {
        let router = Arc::new(Mutex::new(self.router));
        println!("Listening on 127.0.0.1:{}", self.port);
        for stream in self.socket.incoming() {
            match stream {
                Ok(mut s) => {
                    let router = Arc::clone(&router);
                    thread::spawn(move || {
                        let router = router.lock().unwrap();
                        let mut req = request::handle_request(&mut s);
                        let (route, params) = router
                            .get_route(&req.path, &req.method)
                            .expect("Route nto found");
                        req.params = params;
                        route
                            .handle(req)
                            .expect("If you see this, its probably fine");

                        response::handle_response(&mut s);
                    });
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    pub fn route<T>(&mut self, path: impl ToString, method: HttpMethod, handler_fn: T) -> &mut Self
    where
        T: Fn(Request) -> Result<Value, Exception> + Send + Sync + 'static,
    {
        self.router.add_route(crate::Route {
            path: path.to_string(),
            method,
            handler_fn: Box::new(handler_fn),
        });
        self
    }
}
