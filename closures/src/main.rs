use std::thread;
use std::time::Duration;

fn main() {
    generate_workout(22, 3);

    // 函数定义
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    assert_eq!(add_one_v1(1), 2);
    // 有类型标注的闭包
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // 类型推断，仅推断一次，第一次使用时推断
    let add_one_v3 = |x| { x + 1 };
    assert_eq!(add_one_v3(1), 2);
    // 省略大括号
    let add_one_v4 = |x| x + 1;
    assert_eq!(add_one_v4(1), 2);

    add_one_v2(11);

    // 闭包可捕获环境,闭包内访问闭包外的变量
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}


fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}