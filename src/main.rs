use rust_20230407::server::{Handler, ServerEngine};

struct TestHandler;

impl Handler for TestHandler {
    fn service(&self, rq: rust_20230407::http::http_request::Request, mut response: rust_20230407::http::http_response::Response) {
        println!("TestHandler service ......");

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
    }
}

fn main() {
    println!("Hello World!");
    let ip = "127.0.0.1".to_string();
    let port: u32 = 8080;
    let mut serverEngine = ServerEngine::new(ip, port);
    let path = "/";
    let handler = TestHandler;
    serverEngine.add_router(path, &handler);
    serverEngine.start().expect("start engine error!");
}

// use std::net::{TcpListener, TcpStream};
// use std::thread;
// //std::thread库的引入，对输入的每一个流创建一个线程
// use std::time;
// use std::io::{self, Read, Write};
// //引入io库，为了处理错误
//
// fn handle_client(mut stream: TcpStream) -> io::Result<()> {
//     // 该函数用来处理client（就是这个流），流的格式或者说他的类型就是TcpStream
//     let mut buf = [0; 1024 * 4];
//     // 创建一个叫buf的数组，内容为0，长度为512
//     loop {
//         // 该循环表示server端永久提供服务，因为默认服务器为永不关闭的
//         let bytes_read = stream.read(&mut buf)?;
//         // 从流里面读内容，读到buf中
//         if bytes_read == 0 {
//             return Ok(());
//             // 如果读到的为空（即0），则说明已经结束了
//         }
//         println!("request: {}", String::from_utf8_lossy(&buf[..bytes_read]));
//         // stream.write(&buf[..bytes_read])?;
//         // 否则把它写回去
//         thread::sleep(time::Duration::from_secs(1));
//         // 调用sleep函数实现服务的间隔，间隔1s
//     }
// }
//
// fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:8080")?;
//     //定义一个listener，bind函数里面填写的是监听的的ip与端口号,?是一种简写，等价于except,unwrap
//     let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
//     //创建一个容器，用来放线程的句柄
//
//     for stream in listener.incoming() {
//         let stream = stream.expect("failed");
//         //转换一下stream流，出现问题，提示“失败”，没有问题，继续下面的操作
//         let handle = thread::spawn(move || {
//             handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
//         });
//         //对输入的每一个流来创建一个线程，利用必包进行一个处理
//         thread_vec.push(handle);
//         //把handle加到容器里面
//     }
//
//     for handle in thread_vec {
//         //此循环为了等待线程的结束
//         handle.join().unwrap();
//         //等待结束的具体实现
//     }
//     Ok(())
// }
//
//
