pub struct Greeter {
    pub hello_i18n: Vec<String>,
    pub name: String,
}

impl Greeter {
    pub fn say_hello(self) {
        for item in self.hello_i18n {
            println!("{}, {}!", item, self.name)
        }
    }
}

pub fn create_hello() -> Vec<String> {
    let hello_cn = "你好".to_string();
    let hello_en = String::from("hello");
    vec![hello_en, hello_cn]
}
