use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "query: {},file_path: {}", self.query, self.file_path)
    }
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        return Config { query, file_path };
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
    }
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    return Config { query, file_path };
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;
    println!("With text: \n{contents}");
    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

// error
// impl std::fmt::Display for Vec<i32> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!("")
//     }
// }

struct Array(Vec<i32>); // newtype
impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
pub fn demo1() -> String {
    let s = String::from("Hello World!");
    println!("demo1 s p: {:p}", s.as_ptr());
    return s;
}
pub fn demo2(s: &String) {
    println!("s len: {}", s.len());
    println!("demo2 s: {}", s);
}

pub fn demo3(s: &mut String) {
    s.push('c');
}
pub fn demo4() {
    let s = "Hello World!";
    let s1 = &s[0..6];
    let s2 = &s[..6]; // 从0开始 0可省略
    println!("{}", s1);
    println!("{}", s2);
}

pub fn demo5(c: &mut Config) {
    println!("c: {},{:p}", c, &c);
    c.query = String::from("test");
}

pub fn demo() {
    let c = Config {
        query: String::from("1"),
        file_path: String::from("2"),
    };
    let c1 = &c;
    let c2 = &c;
    println!("{}", c1);
    println!("{}", c2);
    // let mut c = Config {
    //     query: String::from("123"),
    //     file_path: String::from("456"),
    // };
    // println!("before c: {},{:p}", c, &c);
    // demo5(&mut c);
    // println!("after c: {},{:p}", c, &c);
    // let mut s = String::from("123");
    // demo3(&mut s);
    // println!("{}",s);
    // let s = demo1();
    // println!("demo s p: {:p}",s.as_ptr());
    // let arr = [1, 2, 3, 4, 5];
    // for i in arr.iter() {
    //     println!("{i}");
    // }

    // let a = [1, 2, 3, 4, 5];
    // for elem in (1..4).rev() {
    //     println!("{elem}"); //3，2，1
    // }
    // let c = Config {
    //     query: "1".parse().unwrap(),
    //     file_path: "1".parse().unwrap(),
    // };
    // let b = Box::new(c);
    // println!("c: {b}");

    // let handle = std::thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("i: {i}");
    //     }
    // });

    // handle.join().unwrap();

    // std::thread::sleep(std::time::Duration::from_millis(1000));

    // let c = Config {
    //     query: String::from("1"),
    //     file_path: String::from("2"),
    // };
    // println!("{:?}", c);
    // println!("{}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
                Rust:
                safe, fast, productive.
                Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
