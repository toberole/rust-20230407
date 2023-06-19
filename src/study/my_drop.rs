struct MyDrop {
    s: String,
}

impl Drop for MyDrop {
    fn drop(&mut self) {
        println!("drop: {}", self.s);
    }
}

pub fn my_drop1() {
    let d = MyDrop { s: String::from("hello") };
    println!("*********************");
}