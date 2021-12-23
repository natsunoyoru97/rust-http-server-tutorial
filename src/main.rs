use std::env;
use std::error::Error;

use tokio::fs;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

static CLDR: &str = "\r\n";

/// 解析并处理客户端的 HTTP 请求
async fn read_http_request(mut stream: TcpStream, buffer: &[u8]) -> Result<(), Box<dyn Error>>{
    // 解析客户端发来的 HTTP 请求
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = httparse::Request::new(&mut headers);
    let res = request.parse(buffer).unwrap();

    //println!("Request: {:?}", request);

    let file_root = "./public";
    if res.is_partial() {
        match request.path {
            Some(ref path) => {
                let mut sub_path = "/index.html";
                // TODO: 检查路由
                if *path != "/" {
                    sub_path = *path;
                }

                //println!("{}", format!("{}{}", file_root, sub_path));

                // TODO: 针对不同 content 类型做不同的处理
                // 目前需要支持图片和 JSON
                let contents = fs::read_to_string(format!("{}{}", file_root, sub_path))
                                            .await?;
                let response = gen_http_response(contents);
                //println!("{}", response);

                stream
                    .write_all(response.as_bytes())
                    .await?;

                stream
                    .flush()
                    .await?;
            },
            None => {
                // must read more and parse again
            }
        }
    }

    Ok(())
}

/// 获取 Content-Type
fn get_content_type<'a>() -> &'a str {
    "text/html"
}

/// 生成 HTTP Response
fn gen_http_response(contents: String) -> String {
    let (status_code, text) = ("200", "OK");
    let status = format!("HTTP/1.1 {0} {1}{2}", status_code, text, CLDR);
    let server_name = format!("Server: Rust{0}", CLDR);
    let content_type = format!("Content-Type: {0};charset=utf-8{1}", get_content_type(), CLDR);
    let content_length = format!("Content-Length:{0}{1}\n", contents.as_bytes().len(), CLDR);

    let response = format!(
        "{0}{1}{2}{3}{4}"
        , status, server_name, content_type, content_length, contents
    );

    response
}

/// 处理已建立的 TCP 连接
async fn handle_tcp_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut message = String::new();
    BufReader::new(&mut stream).read_line(&mut message).await?;
    
    read_http_request(stream, &message.as_bytes()).await?;
    
    Ok(())
}

/// 异步的 HTTP 服务器
#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let server_addr = &args[1];
    let port: u16 = args[2]
                    .parse()
                    .expect("Illegal port");
    println!("IP address {} port {}", &server_addr, port);

    let listener = TcpListener::bind(((*server_addr).clone(), port))
                                .await?;
    
    // listener 可能有多个连接，迭代 listener 拥有的连接
    loop {
        let (stream, _) = listener
                                        .accept()
                                        .await?;
        
        // 为什么异步的反而性能还不如同步的？
        // 一个猜想：读文件的异步操作反而造成了对系统内核的干扰
        tokio::spawn(async move {
            handle_tcp_connection(stream)
                .await
                .expect("Failed to handle the TCP connection");
        });
    }
}