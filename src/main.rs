use std::env;
use std::error::Error;
use std::fs;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// 处理已建立的 TCP 连接
async fn handle_tcp_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut message = String::new();
    BufReader::new(&mut stream).read_line(&mut message).await?;

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
        .write_all(response.as_bytes())
        .await?;

    stream
        .flush()
        .await?;
    
    Ok(())
}

/// 异步的 HTTP 服务器
#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let server_addr = args[1].clone();
    let port: u16 = args[2]
                    .clone()
                    .parse()
                    .expect("Illegal port");
    println!("IP address {} port {}", &server_addr, port);

    let listener = TcpListener::bind((server_addr, port))
                                .await?;
    
    // listener 可能有多个连接，迭代 listener 拥有的连接
    loop {
        let (stream, _) = listener
                                        .accept()
                                        .await?;
        
        tokio::spawn(async move {
            handle_tcp_connection(stream)
                .await
                .expect("Failed to handle the TCP connection");
        });
    }
}