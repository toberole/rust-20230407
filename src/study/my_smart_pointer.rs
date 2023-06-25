use crate::study::my_smart_pointer::List::{Cons, Nil};

pub fn my_smart_pointer1() {
    // 在堆上分配 i32
    let i = Box::new(110);
    println!("i: {}", i);
}

// 错误
// enum List {
//     Cons(i32, List),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn my_smart_pointer2() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("my_smart_pointer2 {:?}",list);
}