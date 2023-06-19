// #[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
// struct MyCopy {
//     name: String,
// }
//
// struct MyCopy1 {
//     name: String,
// }

// impl Clone for MyCopy {
//     fn clone(&self) -> Self {
//         return *self;
//     }
//
//     // fn clone_from(&mut self, source: &Self) where Self: Destruct {
//     //
//     // }
// }

use std::thread::sleep;

struct MyCopyClone {
    s: String,
}

impl Clone for MyCopyClone {
    fn clone(&self) -> Self {
        Self { s: self.s.clone() }
    }
}

// 类型实现了 Copy trait，那么它在变量绑定、函数参数传递、函数返回值传递等场景下，
// 它都是 copy 语义，而不再是默认的 move 语义。

// impl Copy for MyCopyClone {}

#[derive(Clone, Copy)]
struct MyCopyClone1 {
    i: i32,
}

struct MyCopyClone2 {
    i: i32,
}

pub fn test_cpoy_clone() {
    let c1 = MyCopyClone1 { i: 110 };
    // MyCopyClone1 实现了copy 此处发生了copy操作 后续再使用c1不会导致错误
    let c2 = c1;
    println!("{:p},{:p}", &c1.i, &c2.i);

    // let c3 = MyCopyClone2{i:110};
    // // MyCopyClone2 没有实现copy 此处发生了move 后续再使用c3会导致错误
    // let c4 = c3;
    // println!("{:p},{:p}",&c3.i,&c4.i);
}
