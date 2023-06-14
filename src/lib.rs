mod http;

pub mod server {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use crate::http::http_request::Request;
    use crate::http::http_response::Response;

    fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        println!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
        let req = Request::new(&buf[..bytes_read]);
        println!("req: {:?}", req);

        let mut response = Response {
            protocol: "".to_string(),
            code: 0,
            codeDesc: "".to_string(),
            head: Default::default(),
            content: "".to_string(),
        };
        response.set_code(200);
        response.set_codeDesc("OK".to_string());
        response.head.insert("Content-Type".to_string(),
                             "text/html; charset=utf-8".to_string());
        let content = "Hello World!";
        response.set_content(content.to_string());
        response.head.insert("Content-Length".to_string(), content.len().to_string());
        println!("res: {:?}", response);
        let resStr = response.dump();
        stream.write(resStr.as_bytes()).expect("write error!");
        return Ok(());
    }

    pub fn start(ip: &str, port: u32) -> Result<(), String> {
        println!("start server ip: {:?},port: {}", ip, port);

        let s = format!("{}:{}", ip, port);
        println!("ip&&port: {}", s.clone());

        let listener = match TcpListener::bind(s) {
            Ok(l) => l,
            Err(e) => panic!("{:?}", e),
        };

        let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();

        for stream in listener.incoming() {
            let stream = stream.expect("failed!");
            let handle = std::thread::spawn(move || {
                handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
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
}
