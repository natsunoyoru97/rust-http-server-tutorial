use std::env;
use std::fs;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread;

/// 处理已建立的 TCP 连接
fn handle_tcp_connection(mut stream: TcpStream) {
    // peek 不影响缓存，也是推荐方案
    /*
    if stream.peek(&mut buffer).expect("Failed to peek") == 0 {
        return;
    }
    */
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();

    // TODO: 封装这边的逻辑，请求不同的内容返回不同的东西
    let contents = fs::read_to_string("./public/index.html")
                        .unwrap(); // TODO: Handle the error
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Server: Rust\r\n\
        Content-Type: text/html;charset=utf-8\r\n
        Content-Length:{}\r\n\r\n{}"
        , contents.as_bytes().len()
        , contents
    );
    println!("{}", response);

    stream
        .write(response.as_bytes())
        .expect("Write operation failed!");

    stream
        .flush()
        .unwrap();
}

/// 简单的 TCP 服务端，有没有什么办法让它更方便扩展呢？
fn main() {
    let args: Vec<String> = env::args().collect();

    let server_addr = args[1].clone();
    let port: u16 = args[2]
                    .clone()
                    .parse()
                    .expect("Illegal port");
    println!("IP address {} port {}", &server_addr, port);

    let listener = TcpListener::bind((server_addr, port))
                                .expect("Binding address failed!");
    
    // listener 可能有多个连接，迭代 listener 拥有的连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream
                                                .peer_addr()
                                                .unwrap()
                );
                thread::spawn(move|| {
                    handle_tcp_connection(stream);
                });
            },
            Err(e) => {
                println!("Connection failed: {}", e);
            },
        }
    }
}