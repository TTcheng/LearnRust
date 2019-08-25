use std::rc::Rc;

/// Rc<T> 引用计数智能指针
/// 使用 Rc<T> 共享数据
pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}