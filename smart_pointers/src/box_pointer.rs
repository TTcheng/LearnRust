use std::ops::Deref;

//！ 递归的包含将使编译器无法计算需要的存储空间的大小
/*
pub enum List {
    Cons(i32, List),
    Nil,
}
*/

//！ 使用 Box<T> 给递归类型一个已知的大小
pub enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}

/// 自定义智能指针
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/// 为MyBox实现解引用trait以允许使用解引用符
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}