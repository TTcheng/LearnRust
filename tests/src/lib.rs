pub mod rectangle;
pub mod adder;

#[cfg(test)] // 只在cargo test时才编译运行，cargo build不这么做
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use crate::rectangle::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { height: 8, width: 7 };
        let smaller = Rectangle { height: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    use crate::adder::add_two;

    #[test]
    fn another() {
        let res = add_two(2);
        assert_eq!(3, res, "res {} is not equals to given value", res)
    }

    #[test]
    fn not_equal() {
        assert_ne!(3, add_two(2))
    }

    #[test]
    #[should_panic]
    fn panic(){
        panic!("fdasfa")
    }
}
