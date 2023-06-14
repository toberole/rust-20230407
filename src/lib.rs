pub mod http;
pub mod study;
pub mod util;

pub mod server {
    use std::collections::HashMap;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    pub use crate::http::http_handler::Handler;
    use crate::http::http_request::Request;
    use crate::http::http_response::Response;

    // #[derive(Debug)]
    pub struct ServerEngine {
        pub ip: String,
        pub port: u32,
        pub routers: HashMap<String, Box<dyn Handler>>,
    }

    fn handle_client(mut stream: TcpStream, routers: &HashMap<String, Box<dyn Handler>>) -> std::io::Result<()> {
        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        println!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
        let mut req = Request::new(&buf[..bytes_read]);
        println!("req: {:?}", req);
        req.set_stream(Some(stream));

        let mut response = Response {
            protocol: "".to_string(),
            code: 0,
            codeDesc: "".to_string(),
            head: Default::default(),
            content: "".to_string(),
        };

        if routers.contains_key(req.path()) {
            let handler = &routers.get(req.path());
            match handler {
                Some(h) => {
                    h.service(req, response)
                }
                None => {
                    println!("not found router!");
                    return Ok(());
                }
            }
        }
        return Ok(());
    }

    unsafe impl Send for ServerEngine {}

    impl ServerEngine {
        pub fn start(&self) -> Result<(), String> {
            println!("start server ip: {:?},port: {}", self.ip, self.port);

            let s = format!("{}:{}", self.ip, self.port);
            println!("ip&&port: {}", s.clone());

            let listener = match TcpListener::bind(s) {
                Ok(l) => l,
                Err(e) => panic!("{:?}", e),
            };

            let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();

            for stream in listener.incoming() {
                let stream = stream.expect("failed!");
                let routers = &self.routers;
                let handle = std::thread::spawn(move || {
                    // handle_client(stream, &routers).unwrap_or_else(|error| eprintln!("{:?}", error));
                });

                thread_vec.push(handle);
            }

            for handle in thread_vec {
                handle.join().unwrap();
            }
            return Ok(());
        }

        pub fn stop() -> Result<(), String> {
            println!("stop server");
            return Ok(());
        }

        pub fn new(ip: String, port: u32) -> ServerEngine {
            let engine = ServerEngine {
                ip,
                port,
                routers: Default::default(),
            };
            return engine;
        }
        pub fn set_ip(&mut self, ip: String) {
            self.ip = ip;
        }
        pub fn set_port(&mut self, port: u32) {
            self.port = port;
        }
        pub fn add_router(&mut self, path: &str, handler: &dyn Handler) {
            // self.routers.insert(path.to_string(), Box::new(handler));
        }
    }
}
