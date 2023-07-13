/**/
pub fn my_lambda1(){
    let f = ||->i32{
        return 1;
    };

    f();


}

pub fn my_lambda2(i:&i32){
    let n = 0;
    let n = n+i;
    println!("{}",n);
}