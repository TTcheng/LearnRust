/// 使用 Drop Trait 运行清理代码
pub struct CustomSmartPointer {
   pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}