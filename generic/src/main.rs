fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("{:?}", both_integer);
    println!("{:?}", integer_and_float.tuple());
    println!("x:{},y:{}", both_float.x(), both_float.y());
}

// generic struct
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// generic enum
enum Option<T> {
    Some(T),
    None,
}

// generic function
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
    fn tuple(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
