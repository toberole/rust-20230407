use rust_20230407::{
    public_module::public_function,
    test::{play1, test1},
};

use m1::m2::Test1;

pub mod m1 {
    pub mod m2 {
        pub fn Test1() {
            println!("Test1 ......");
        }
    }
}

fn main() {
    println!("Hello World!");
    // public_function();
    // Test1();
    test1();
    // play1();

    // println!("++++++++++++++++++++");
    // let v = vec![10, 20, 30];
    // let n = 10;
    // if v.contains(&n) {
    //     println!("found 10");
    // }
    // println!("{:?}", v);
}
