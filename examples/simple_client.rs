use std::env;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

/// 简单的 TCP 客户端。有没有什么办法让它更方便扩展呢？
fn main() {
    let args: Vec<String> = env::args().collect();

    let client_addr = args[1].clone();
    let port: u16 = args[2].clone().parse().expect("端口不合法");

    match TcpStream::connect((client_addr.clone(), port)) {
        Ok(mut stream) => {
            println!("成功连接到{}:{}", client_addr.clone(), port);

            let msg = b"Hello!";

            stream.write(msg).expect("写操作失败！");
            println!("已发送 {:?}，等待回复中……", msg);

            let mut data = [0 as u8; 512];
            match stream.read(&mut data) {
                Ok(_) => println!("对方已回复：{:?}", data),
                Err(e) => println!("无法获取资源：{}", e),
            }
        },
        Err(e) => eprintln!("连接失败： {}", e),
    }
    
}