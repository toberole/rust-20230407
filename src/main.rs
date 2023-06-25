#![allow(dead_code, unused_imports)]
// #[allow(dead_code)]: 随时需要随时添加，添加到告警行代码的上方，只生效当前代码。若编写lib的时候，有些代码是为了导出给他人调用，可使用该方法
// #![allow(dead_code, unused_imports)]: 一次添加，整体有效。添加到lib.rs或者main.rs的首行，可以使整个程序不再有告警。

extern crate rust_20230407;

use rust_20230407::study::demo1::demo1_1;
use rust_20230407::study::my_copy_clone::test_cpoy_clone;
use rust_20230407::study::my_drop::my_drop1;
use rust_20230407::study::my_dyn_trait::dyn_trait1;
use rust_20230407::study::my_enum::my_enum1;
use rust_20230407::study::my_error::my_error1;
use rust_20230407::study::my_lifetime_mark::my_lifetime_mark_1;
use rust_20230407::study::my_map::my_map1;
use rust_20230407::study::my_slice::my_slice1;
use rust_20230407::study::my_smart_pointer::{my_smart_pointer1, my_smart_pointer2};
use rust_20230407::study::my_trait::my_trait1;

fn main() {
    println!("Hello World!");
    // demo1_1();
    // dyn_trait1();
    // my_error1();
    // my_map1();
    // my_trait1();
    // my_lifetime_mark_1();
    // my_drop1();
    // test_cpoy_clone();
    // my_slice1();
    // my_enum1();
    my_smart_pointer2();
}
