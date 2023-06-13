pub mod test;
//公开的模块
pub mod public_module {
    pub fn public_function() {
        // 公开的方法
        println!("public function");
    }
    fn private_function() {
        // 私有的方法
        println!("private function");
    }
}
//私有的模块
mod private_module {
    // 私有的方法
    fn private_function() {
        println!("private module && private function");
    }
}
