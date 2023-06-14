use std::os::unix::raw::time_t;

trait MyTrait {
    fn hello(&self);
}

trait MyTrait1 {
    fn hello1(&self);
}

fn testTrait(t: impl MyTrait) {
    t.hello();
}

fn testTrait1<T: MyTrait>(t: T) {
    t.hello();
}

fn testTrait2(t: impl MyTrait, t1: impl MyTrait1) {
    t.hello();
    t1.hello1();
}

fn testTrait3<T: MyTrait, T1: MyTrait1>(t: T, t1: T1) {
    t.hello();
    t1.hello1();
}

fn testTrait4<T: MyTrait + MyTrait1>(t: T) {
    // t.hello();
}

fn testTrait5(t: impl MyTrait + MyTrait1) {
    // t.hello();
}

struct MyTraitDemo;

impl MyTrait for MyTraitDemo {
    fn hello(&self) {
        println!("MyTraitDemo hello")
    }
}

pub fn my_trait1() {
    let m = MyTraitDemo;
    m.hello();
}