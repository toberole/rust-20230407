use std::collections::HashMap;
use crate::util::print_type_of;

// fn dangle() -> &String { // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// }

fn my_map1_1(s: String) {
    println!("{:p}", s.as_ptr());
}

fn my_map1_2() {
    let v = vec![1, 2, 3];
    println!("before {:p}", v.as_ptr());
    let handle = std::thread::spawn(move || {
        println!("move {:p}", v.as_ptr());
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn my_map1() {
    let mut map = HashMap::new();
    map.insert("Hello", "World!");
    let mut b = map.contains_key("Hello");
    println!("{}", b);
    // map.remove("Hello");
    // b = map.contains_key("Hello");
    // println!("{}", b);

    let v: Vec<(_, _)> = map.into_iter().collect();
    println!("{:?}", v);

    let s = String::from("123");
    // println!("before {:p}", s.as_ptr());
    // print_type_of(&s.as_ptr());
    // my_map1_1(s);
    my_map1_2();
}