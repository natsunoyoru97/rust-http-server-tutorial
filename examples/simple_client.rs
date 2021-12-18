use std::env;
use std::net::TcpStream;
use std::io;
use std::io::{Read, Write};

/// 简单的 TCP 客户端。有没有什么办法让它更方便扩展呢？
fn main() {
    let args: Vec<String> = env::args().collect();

    let client_addr = args[1].clone();
    let port: u16 = args[2].clone().parse().expect("Illegal port");

    while match TcpStream::connect((client_addr.clone(), port)) {
        Ok(mut stream) => {
            println!("Already connect to {}:{}", client_addr.clone(), port);

            let mut msg = String::new();
            io::stdin().read_line(&mut msg).expect("Error in reading message");

            println!("Sent {:?} to the server, waiting for response ...", msg);
            stream.write(&mut msg.into_bytes()).expect("Failed to write");

            let mut data = vec![0 as u8; 16];
            match stream.read(&mut data) {
                Ok(_) => {
                    println!("Response from the server: {:?}", String::from_utf8(data));
                    true
                },
                Err(e) => {
                    println!("{}", e);
                    false
                },
            }
        },
        Err(e) => {
            eprintln!("Failed to connect {}", e);
            false
        },
    } {}
    
}