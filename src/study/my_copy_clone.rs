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

impl Copy for MyCopyClone {}