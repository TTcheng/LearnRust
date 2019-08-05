use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");

        // mut 指定可变变量，没有mut修饰的则为常量
        let mut guess = String::new();

        // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim消除不可见字符、parse类型转换、
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
