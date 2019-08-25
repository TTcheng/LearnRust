use std::rc::Rc;

/// 智能指针是一个在 Rust 经常被使用的通用设计模式
///
/// **常用的智能指针如下**
/// - Box<T>，用于在堆上分配值,并且可确定大小
/// - Rc<T>，一个引用计数类型，其数据可以有多个所有者
/// - Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型。

/// **如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：**
///
/// Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
/// Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
/// 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

pub mod box_pointer;
pub mod use_drop_trait;
pub mod rc_pointer;

fn main() {
    /// box在堆上存储数据
    let b = Box::new(1);
    println!("b={}", b);
    /// 下面的代码将无法通过编译，递归的包含将使编译器无法计算需要的存储空间的大小
    // use box_pointer::List;
    // let a = List::Cons(1, List::Nil);
    /// 使用 Box<T> 给递归类型一个已知的大小
    use box_pointer::ListBox::*;
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    /// Box实现了`Deref trait`，可以直接使用解引用符号`*`
    assert_eq!(1, *b);
    /// 使用自定义智能指针
    use crate::box_pointer::MyBox;
    let five = MyBox::new(5);
    assert_eq!(5, *five);

    /// 使用`Drop Trait`在清理前运行代码
    use crate::use_drop_trait::*;
    let c = CustomSmartPointer { data: String::from("my stuff") };
    // 通过 std::mem::drop 提早丢弃值
    drop(c);
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    use crate::rc_pointer::RcList;
    /// 使用 Rc<T> 共享数据
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("after creating a，count= {}", Rc::strong_count(&a));
    /// 克隆 Rc<T> 会增加引用计数, 而不是深拷贝
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("after creating b，count = {}", Rc::strong_count(&a));
    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("after creating c，count = {}", Rc::strong_count(&a));
    }
    println!("after c goes out of scope，count = {}", Rc::strong_count(&a));
    // 丢弃变量
}
