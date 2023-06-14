use std::net::{TcpListener, TcpStream};
use std::thread;
//std::thread库的引入，对输入的每一个流创建一个线程
use std::time;
use std::io::{self, BufRead, BufReader, Read, Write};
//引入io库，为了处理错误

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //该函数用来处理client（就是这个流），流的格式或者说他的类型就是TcpStream
    let mut buf = [0; 512];
    //创建一个叫buf的数组，内容为0，长度为512
    loop {
        //该循环表示server端永久提供服务，因为默认服务器为永不关闭的
        let bytes_read = stream.read(&mut buf)?;
        //从流里面读内容，读到buf中
        if bytes_read == 0 {
            return Ok(());
            //如果读到的为空（即0），则说明已经结束了
        }
        stream.write(&buf[..bytes_read])?;
        //否则把它写回去
        thread::sleep(time::Duration::from_secs(1));
        //调用sleep函数实现服务的间隔，间隔1s
    }
}

pub fn server_start() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //定义一个listener，bind函数里面填写的是监听的的ip与端口号,?是一种简写，等价于except,unwrap
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //创建一个容器，用来放线程的句柄

    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        //转换一下stream流，出现问题，提示“失败”，没有问题，继续下面的操作
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        //对输入的每一个流来创建一个线程，利用必包进行一个处理
        thread_vec.push(handle);
        //把handle加到容器里面
    }

    for handle in thread_vec {
        //此循环为了等待线程的结束
        handle.join().unwrap();
        //等待结束的具体实现
    }
    Ok(())
}

pub fn client_start() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    //创建变量stream，直接连接sever端
    for _ in 0..10 {
        let mut input = String::new();
        //定义一个String类型的输入
        io::stdin().read_line(&mut input).expect("Failed to read!");
        //从标准输入读入一行，读入input里面，如果有问题的话，提示“读取失败”
        stream.write(input.as_bytes()).expect("Failed to write!");
        //把input读取的内容，转换成bytes后，写到stream流里面去，如果写入失败，提示“写入失败”

        let mut reader = BufReader::new(&stream);
        //从stream流创建一个读，目的是要从我们的server端读，
        let mut buffer: Vec<u8> = Vec::new();
        //用Vector创建一个buffer变量
        reader.read_until(b'\n', &mut buffer).expect("Failed to read into buffer");
        // 一直读到换行为止（b'\n'中的b表示字节），读到buffer里面
        // println!("read from server: {}", str::from_utf8(&buffer).unwrap());
        // 把读取到buffer中的内容打印出来
        println!("");
        //再来一个换行，美化输出
    }
    Ok(())
}


