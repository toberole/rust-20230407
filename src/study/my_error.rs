// 实现一个最基本的自定义错误只需要实现下面两个trait，这两个都是关于把错误信息输出的。
// 手动实现impl std::fmt::Debug的trait，一般直接添加注解即可：#[derive(Debug)]
// 手动实现impl std::fmt::Display的trait,，用于自定义输出错误文本信息。

use std::fmt::{Display, Formatter};
use std::{io, num};
use std::fs::File;
use std::io::{Error, Read};
use std::num::ParseIntError;

#[derive(Debug)]
struct AppError {
    kind: String,
    msg: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "kind: {},msg: {}", self.kind, self.msg);
    }
}

// 实现From::from<io::Error>的trait
// 这样AppError就能接住io::Error类型的error
impl From<io::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError {
            kind: String::from("io_error"),
            msg: value.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        AppError {
            kind: String::from("parse_error"),
            msg: value.to_string(),
        }
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError {
        kind: String::from("404"),
        msg: String::from("page not found"),
    })
}

pub fn my_error1() {
    match produce_error() {
        Err(e) => {
            println!("error: {:?}", e);
        }
        Ok(()) => {
            println!("ok");
        }
    }
}

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn my_error2() {
    /*
    类 Java的try ... catch
    */
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                }
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}
