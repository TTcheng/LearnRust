/// 智能指针是一个在 Rust 经常被使用的通用设计模式
///
/// **常用的智能指针如下**
/// - Box<T>，用于在堆上分配值,并且可确定大小
/// - Rc<T>，一个引用计数类型，其数据可以有多个所有者
/// - Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型。

pub mod box_pointer;

fn main() {
    /// box在堆上存储数据
    let b = Box::new(1);
    println!("b={}", b);
    /// 下面的代码将无法通过编译，递归的包含将使编译器无法计算需要的存储空间的大小
    use box_pointer::List;
    // let a = List::Cons(1, List::Nil);
    /// 使用 Box<T> 给递归类型一个已知的大小
    use box_pointer::ListBox::*;
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


}
