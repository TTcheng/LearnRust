pub fn use_vector() -> (Vec<u32>, Vec<i32>, Vec<Types>) {
    // 使用函数创建
    let mut v1: Vec<u32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);

    // 使用宏
    let mut v2 = vec![1, 2, 3];
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);

    // 使用枚举存储多种类型、
    let v3 = vec![Types::Int(1), Types::Text("hello".to_string())];

    (v1, v2, v3)
}

pub fn print_vec(vector: Vec<Types>) {
    for item in vector {
        match item {
            Types::Int(num) => println!("{}", num),
            Types::Text(str) => println!("{}", str),
        }
    }
}

pub enum Types {
    Int(i32),
    Text(String),
}
