fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 类单元结构体
struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

// 元组结构体
struct WriteMessage(String);

// 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体