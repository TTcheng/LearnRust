pub mod largest;

use largest::largest;
use std::fmt::{Display, Debug};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    // largest
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

//通过 where 简化代码
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}

// trait作为返回类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // 不允许返回不同类型，比如某些时候返回Tweet某些时候返回NewArticle
}

// trait作为参数
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound
pub fn notify1<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// multi trait
pub fn notify_multi(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

// multi trait bound
pub fn notify_multi1<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// trait类似接口
pub trait Summary {
    // 默认实现
    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// 有条件的设计
impl<T: Display + PartialOrd> Pair<T> {
    // 实现了Display和PartialOrd时才实现下面的方法
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
