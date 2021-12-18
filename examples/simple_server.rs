use std::env;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;

/// 处理已建立的 TCP 连接
fn handle_tcp_connection(mut stream: TcpStream) {
    let mut buffer = vec![0 as u8; 16];
    match stream.read(&mut buffer) {
        Ok(size) => { 
            println!("Get message from the client: {:?}, start to response", buffer);
            stream.write(&buffer[0..size]).expect("Write operation failed!");
        },
        Err(_) => {
            stream.shutdown(Shutdown::Both).expect("Failed to shutdown!");
        },
    } {}
}

/// 简单的 TCP 服务端，有没有什么办法让它更方便扩展呢？
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let server_addr = args[1].clone();
    let port: u16 = args[2]
                    .clone()
                    .parse()
                    .expect("端口不合法");
    println!("IP 地址：{:?} 端口：{:?}", &server_addr, port);

    let listener = TcpListener::bind((server_addr, port))
                                .expect("Binding address failed!");
    
    // listener 可能有多个连接，迭代 listener 拥有的连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_tcp_connection(stream)
                });
            },
            Err(e) => {
                println!("Connection failed: {}", e);
            },
        }
    }

    drop(listener);

    Ok(())
}