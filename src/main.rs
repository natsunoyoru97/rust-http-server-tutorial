use image::io::Reader as ImageReader;

use std::env;
use std::error::Error;
use std::str;

use tokio::fs;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

static CLDR: &str = "\r\n";

/// 解析并处理客户端的 HTTP 请求
async fn read_http_request(mut stream: TcpStream, buffer: &[u8]) -> Result<(), Box<dyn Error>> {
    // 解析客户端发来的 HTTP 请求
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = httparse::Request::new(&mut headers);
    let res = request.parse(buffer).unwrap();

    let file_root = "./public";

    if res.is_partial() {
        match request.path {
            Some(path) => {
                let sub_path = match path {
                    "/" => "/index.html",
                    _ => path,
                };
                let file_vec = sub_path
                                        .split('.')
                                        .collect::<Vec<&str>>();
                let file_type = file_vec
                                        .last()
                                        .unwrap_or(&"");

                println!("{}", format!("{},{}", file_root, sub_path));

                // TODO: 针对不同 content 类型做不同的处理
                // 目前需要支持 png 格式以外的图片和 JSON
                println!("{}", *file_type);
                let mut bytes: Vec<u8> = Vec::new();
                let contents = match *file_type {
                    "html" => {
                        fs::read(format!("{0}{1}", file_root, sub_path))
                                            .await?
                    },
                    "png" => {
                        let img = ImageReader::open(format!("{0}{1}", file_root, sub_path))?
                                                .decode()?;
                        img.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
                        
                        bytes
                    },
                    _ => bytes,
                };
                
                let response = gen_http_response(contents.as_slice(), file_type);
                stream
                    .write_all(response.as_slice())
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
fn get_content_type<'a>(file_type: &'a str) -> &'a str {
    // TODO: 添加更多文件格式
    match file_type {
        "html" => "text/html",
        "png" => "image/png",
        _ => "",
    }
}

/// 生成 HTTP Response
fn gen_http_response(contents: &[u8], file_type: &str) -> Vec<u8> {
    let (status_code, text) = ("200", "OK");
    let status = format!("HTTP/1.1 {0} {1}{2}", status_code, text, CLDR);
    let server_name = format!("Server: Rust{0}", CLDR);
    let content_type = format!("Content-Type: {0};", get_content_type(file_type));
    //let char_set = format!("charset=utf-8{0}", CLDR);
    let content_length = format!("Content-Length:{0}{1}\n", contents.len(), CLDR);

    let response = format!(
        "{0}{1}{2}{3}"
        , status, server_name, content_type, content_length
    );
    println!("{}", response);

    [response.as_bytes(), contents].concat()
}

/// 处理已建立的 TCP 连接
async fn handle_tcp_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut message = String::new();
    BufReader::new(&mut stream).read_line(&mut message).await?;
    println!("{}", message);
    
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
        
        tokio::spawn(async move {
            handle_tcp_connection(stream)
                .await
                .expect("Failed to handle the TCP connection");
        });
    }
}