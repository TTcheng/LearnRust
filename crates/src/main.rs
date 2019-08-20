pub mod art;
// 使用重导出
use art::*;
// 使用原始导出
//use art::kinds::PrimaryColor;
//use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
// 文档注释如下


/// 将给定的数字加一
///
/// # Example
/// ```
/// let five = 5;
/// assert_eq!(6, add_one(five))
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}