use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // 创建闭包
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 使用 join 等待所有线程结束
    // handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。
    handle.join().unwrap();

    // move闭包获取环境值的所有权，常规闭包是借用，但是在新的线程中无法确定引用是否有效
    let v = vec![1, 2, 3];
    // v moved here
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    // 消息传递
    // 不要共享内存来通讯；而是要通讯来共享数据
    let (tx, rx) = mpsc::channel();
    // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
    // 通过克隆发送者来创建多个生产者
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        // vals moved here, 通道会转移所有权
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        // vals moved here, 通道会转移所有权
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}
