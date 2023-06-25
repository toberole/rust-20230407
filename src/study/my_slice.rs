pub fn my_slice1() {
    // ..y 等价于 0..y
    // x.. 等价于位置 x 到数据结束
    // ..  等价于位置 0 到结束
    println!("Hello my_slice1 ......");
    let s = String::from("broadcast");
    let s1 = &s[0..5];
    let s2 = &s[5..9];
    println!("{}={}+{}", s, s1, s2);

    let s3 = "hello world!".to_string();
    let n1 = s3.capacity();
    let len1 = s3.len();
    println!("String {},{}", n1, len1);

    // 将 String 转换成 &str
    let s4 = String::from("Hello world!");
    let s5 = &s4[..];
    println!("s4: {},s5: {}", s4, s5);

    let arr = [1, 2, 3, 4, 5];
    let arr1 = &arr[0..4];
    for i in arr1.iter() {
        println!("i: {}", i);
    }
}