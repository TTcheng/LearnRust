mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // 相对路径
            super::super::breathe_in();
        }
    }
}

fn main() {
    // 绝对路径
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();

    // 使用use将模块引入作用域
    use sound::instrument;
    instrument::clarinet();

    use menu::Appetizer::*;
    let order1 = Soup;
    let order2 = Salad;

    match order1 {
        Soup => println!("Appetizer"),
        Salad => println!("Salad")
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn breathe_in() {
    // 函数体
}