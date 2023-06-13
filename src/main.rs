// use std::error::Error;
// use std::{env, fs};

use rust_20230407::demo;

fn main() {
    println!("hello world!");
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let query = &args[1];
    // let file_path = &args[2];
    // println!("Searching for {}", query);
    // println!("In file {}", file_path);
    //
    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");
    // println!("With text:\n{contents}");

    // let (query, file_path) = parse_config(&args);
    // let config = Config::new(&args);
    /*
    unwrap_or_else，它定义于标准库的 Result<T, E> 上。
    使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理。
    当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。
    然而，当其值是 Err 时，该方法会调用一个 闭包（closure），
    也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数
     */
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     std::process::exit(1);
    // });

    // if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     std::process::exit(1);
    // };
    demo();
}

// fn run(config: Config) {
//     let contents = std::fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");
//     println!("With text: \n{contents}");
// }

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];
//     return (query, file_path);
// }
