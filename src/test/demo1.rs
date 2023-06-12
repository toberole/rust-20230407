use std::{collections::HashMap, sync::mpsc};

pub fn demo4<'a, 'b>(i: &'a i32, i1: &'b i32) {
    println!("{},{}", i, i1);
}

pub fn demo3() {
    let mut v: Vec<i32> = Vec::new();
    v.insert(0, 100);
    v.push(1);
    v.push(2);

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = &v1[1];
    println!("v1[1]: {}", v2);

    let mut map = HashMap::new();
    map.insert("1", "1");
    let vk = map.get("1");
    println!("vk: {:?}", vk);
}

pub fn demo(s: &str) -> Option<&str> {
    match s {
        "a" => Some("a+1"),
        _ => None,
    }
}

pub fn demo2() {
    let (s, r) = mpsc::channel();

    std::thread::spawn(move || {
        let temp = String::from("Hello Thread Spawn!");
        s.send(temp).unwrap();
    });

    let temp = r.recv().unwrap();
    println!("temp: {}", temp);

    // 重影
    let x = 1;
    let x = 11;
    println!("x: {}", x);

    // 重影与可变变量的赋值不是一个概念，
    // 重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。
    // 但可变变量赋值仅能发生值的变化。
    // let mut s = "123";
    // s = s.len();
}

fn demo_1() -> Result<String, i32> {
    let i = 1;
    if i % 2 == 0 {
        return Ok("Hello demo_1".to_string());
    } else {
        return Err(-1);
    }
}
