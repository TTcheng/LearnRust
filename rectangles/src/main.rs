fn main() {
    let rectangle = Rectangle { width: 30, height: 50 };
    println!("rectangle is {:#?}", rectangle);
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
