use crate::errors::NotFoundException;
use crate::request;
use crate::request::Request;
use crate::response;
use crate::response::Result;
use crate::router::Router;
use crate::HttpMethod;
use crate::Route;
use serde::Serialize;
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
    pub fn new(port: u32) -> std::result::Result<Self, std::io::Error> {
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

                        match router.get_route(&req.path, &req.method) {
                            Some((route, params)) => {
                                req.params = params;
                                let res = route.handle(req);

                                response::handle_response(&mut s, res).expect("Unfortunate");
                            }
                            None => {
                                let exc = NotFoundException::new("Not Found");

                                response::handle_response(&mut s, Err(exc.into()))
                                    .expect("Unfortunate");
                            }
                        }
                    });
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    pub fn route<T, F>(&mut self, path: impl ToString, method: HttpMethod, handler: F) -> &mut Self
    where
        T: Serialize + 'static,
        F: Fn(Request) -> Result<T> + Send + Sync + 'static,
    {
        self.router
            .add_route(Route::new(path.to_string(), method, handler));
        self
    }
}
