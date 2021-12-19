use std::env;
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
    loop {
        let mut buffer = vec![0 as u8; 512];
        match stream.read(&mut buffer) {
            Ok(size) => { 
                if size == 0 {
                    return;
                }
                let msg = String::from_utf8(buffer.clone())
                                    .expect("Error in parsing data from the client");
                let msg_temp = msg.trim_matches(char::from(0));
                println!("Get message from the client: {}, start to response"
                        , msg);

                println!("{:?}", str::to_ascii_lowercase(msg_temp).eq("exit"));

                if str::to_ascii_lowercase(msg_temp).eq("exit") {
                    println!("Already told the client to close connection");
                    return ();
                }
                else {
                    stream
                        .write(&buffer[0..size])
                        .expect("Write operation failed!");
                    println!("Already sent response to the client");
                }

            },
            Err(_) => {
                stream.shutdown(Shutdown::Both)
                    .expect("Failed to shutdown!");

            },
        }

        stream
            .flush()
            .unwrap();
        println!("{}", String::from_utf8(buffer.clone())
                                    .expect("Error in parsing data from the client"));
    }
}

/// 简单的 TCP 服务端，有没有什么办法让它更方便扩展呢？
fn main() -> std::io::Result<()> {
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
                                                .unwrap());
                thread::spawn(move|| {
                    handle_tcp_connection(stream);
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