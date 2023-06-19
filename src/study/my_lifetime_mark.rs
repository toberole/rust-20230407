// pub fn my_lifetime_mark_1()->&str{// error 缺乏生命周期
//
// }
// 函数的引用参数和引用返回值需要生命周期参数进行标注，
// 帮助编译器建立和编译时检查引用之间的生命周期约束关系。
// 返回引用的生命周期，需要依赖于输入参数的生命周期，
// 或者是整个程序的运行周期（即：静态生命周期'static）
pub fn my_lifetime_mark_1() -> &'static str {// error 缺乏生命周期
    let hello = "Hello";
    return hello;
}