// 使用trait修复前面遇到的largest函数
// 传递给largest的T必须实现Partial + Copy
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}