fn main() {
    assert_eq!(value_in_cents(Coin::Penny), 1);
    let five = Some(5);
    assert_eq!(5, five.unwrap());
    // if let 简单控制流
    if let Some(5) = five {
        println!("five")
    } else {
        println!("not five")
    }
}

fn plus_one(num: Option<u32>) -> Option<u32> {
    match num {
        Some(n) => Some(n + 1),
        None => None
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}