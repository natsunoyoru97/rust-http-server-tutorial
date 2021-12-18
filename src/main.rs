use std::env;
use std::io::{
    Read, Write
};
use std::net::{TcpListener, TcpStream};

/// 处理已经 accept 的 TCP 请求
pub struct TcpHandler {

}

impl TcpHandler {
    /// 处理已建立的 TCP 连接
    fn handle_tcp_connection(mut stream: TcpStream) {
        println!("新客户端：{:?}", stream.local_addr());

        //let mut buffer = [0; 1024];
        let hello_zh = b"\xE4\xBD\xA0\xE5\xA5\xBD\xEF\xBC\x81";
        stream.write(hello_zh).expect("无法读取 TCP 数据！");
    }
}

/// ### 用 Rust 实现一个 HTTP 服务器
/// - I/O 模型：Linux aio（异步、非阻塞）
/// - 支持线程池
/// - Proactor / Reactor 线程模型，二者将封装成模块，可随意调用
/// 
/// [参考](https://www.zupzup.org/epoll-with-rust/index.html)
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let server_addr = args[1].clone();
    let port: u16 = args[2].clone().parse().expect("端口不合法");
    println!("IP 地址：{:?} 端口：{:?}", &server_addr, port);

    let listener = TcpListener::bind((server_addr, port))?;
    
    // listener 可能有多个连接，迭代 listener 拥有的连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => TcpHandler::handle_tcp_connection(stream),
            Err(e) => eprintln!("无法找到客户端：{}", e),
        }
    }

    Ok(())
}
