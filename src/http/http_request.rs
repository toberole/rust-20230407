use std::collections::HashMap;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    method: String,
    protocol: String,
    path: String,
    head: HashMap<String, String>,
    content: String,
    stream: Option<TcpStream>,
}

impl Request {
    pub fn new(bytes: &[u8]) -> Request {
        let mut req = Request {
            method: "".to_string(),
            protocol: "".to_string(),
            path: "".to_string(),
            head: Default::default(),
            content: "".to_string(),
            stream: None,
        };
        let content = String::from_utf8_lossy(bytes);
        let lines: Vec<&str> = content.split("\n").collect();
        let mut line_n = 0;
        for line in lines {
            println!("line: {}", line);
            let line = line.trim();
            if line.len() <= 0 {
                continue;
            }

            line_n = line_n + 1;
            if line_n == 1 {
                // GET / HTTP/1.1
                let strs: Vec<&str> = line.split(" ").collect();
                if strs.len() < 3 {
                    continue;
                }

                req.method = strs.get(0).unwrap().to_string();
                req.path = strs.get(1).unwrap().to_string();
                req.protocol = strs.get(2).unwrap().to_string();
            } else {
                // Host: 127.0.0.1:8080
                let kv: Vec<&str> = line.split(": ").collect();
                if kv.len() < 2 {
                    continue;
                }

                req.head.insert(kv.get(0).unwrap().to_string(), kv.get(1).unwrap().to_string());
            }
        }
        return req;
    }
    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }
    pub fn set_protocol(&mut self, protocol: String) {
        self.protocol = protocol;
    }
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
    pub fn set_head(&mut self, head: HashMap<String, String>) {
        self.head = head;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
    pub fn set_stream(&mut self, stream: Option<TcpStream>) {
        self.stream = stream;
    }
    pub fn method(&self) -> &str {
        &self.method
    }
    pub fn protocol(&self) -> &str {
        &self.protocol
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn head(&self) -> &HashMap<String, String> {
        &self.head
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn stream(&self) -> &Option<TcpStream> {
        &self.stream
    }
}
