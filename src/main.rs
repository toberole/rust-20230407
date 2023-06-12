use crate::{garden::vegetables::Asparagus, test::demo1};

// 告诉编译器应该包含在src/garden.rs文件中发现的代码
pub mod garden;
pub mod test;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    // let s1 = match demo1::demo("a") {
    //     Some(s) => {s},
    //     None => {"no ......"}
    // };
    // println!("demo1::demo {}",s1);

    demo1::demo3();
}
