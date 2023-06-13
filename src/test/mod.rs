pub mod play;
pub use crate::test::play::play1; // 对外导出

pub fn test1() {
    let s = String::from("hello world!");
    for c in s.chars() {
        println!("{}", c);
    }
    // println!("test1 ......");
    // play1();

    // let s = "hello world!".to_string();
    // println!("{}", s);

    // let hello = "Здравствуйте";
    // let s = &hello[0..4]; // s 为 &str,包含字符串的头四个字节
    // let s2 = &hello[0..1]; // Rust 在运行时会 panic
}
