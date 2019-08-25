//！ 递归的包含将使编译器无法计算需要的存储空间的大小
pub enum List {
    Cons(i32, List),
    Nil,
}

//！ 使用 Box<T> 给递归类型一个已知的大小
pub enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}
