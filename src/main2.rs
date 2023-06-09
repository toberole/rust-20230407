use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

const ONE_DATA: u64 = 24 * 60 * 60 * 1000;

fn test1() {
    println!("please input your guess.");
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {secret_number}");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");
        /**
            用一个新值来 隐藏 （Shadowing） guess 之前的值。
            这个功能常用在需要转换值类型之类的场景。
            它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量
        **/
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("you guessed: {guess}");
    }
}

fn test2() {
    let n = 1;
    if n > 0 {
        println!("1 > 0");
    } else if n < 1 {
        println!("n<1");
    }

    let mut s = String::from("Hello World!");
    s.push_str(",123");
    println!("{}", s);
}

fn test3_1(s: String) {
    println!("s: {}", s);
}

fn test3() {
    // let mut s = String::from("Hello");
    // s.push_str(",World!");
    // println!("{s}");
    //
    // let mut ss = s;
    // println!(ss)
    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{}, world!", s1);
    let s = String::from("12345");
    test3_1(s.clone());
    println!("s: {}", s);
}

fn create() -> String {
    let s = String::from("hello");
    return s; // 所有权转移，从函数内部移动到外部
}

fn consume(s: String) {
    // 所有权转移，从函数外部移动到内部
    println!("{}", s);
}

fn test4() {
    let a = 1;
    let b = &a;
    let c = a + *b;
    println!("test4 {}", c);
}
fn test5() {
    let needle = 42;
    let arr = [1, 2, 3, 4, 5];
    for item in &arr {
        let res = match item {
            2 => "hits",
            _ => "miss",
        };

        println!("res: {}", res);
    }
}

fn test6() -> String {
    return String::from("Hello");
}

fn test7(a: i32, b: i32) -> i32 {
    println!("a: {},b: {}", a, b);
    return a + b;
}

fn main1() {
    println!("Hello, Rust!");
    test7(1, 2);
}
