pub fn my_slice1() {
    println!("Hello my_slice1 ......");
    let s = String::from("broadcast");
    let s1 = &s[0..5];
    let s2 = &s[5..9];
    println!("{}={}+{}", s, s1, s2);
}