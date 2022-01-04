//! 使用 tokio 运行时搭建的异步 HTTP 服务器
//! tokio 运行时已经封装好了线程池、executor 和 poll/epoll
use clap::Parser;
use image::{
    imageops::FilterType,
    io::Reader as ImageReader
};
use std::{
    error::Error,
    str
};
use tokio::{
    fs,
    net::{TcpListener, TcpStream},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}
};

static CLDR: &str = "\r\n";

#[derive(Parser, Debug)]
#[clap(about = "A simple Rust HTTP server", version = "0.1.0", author = "natsunoyoru97 <natsunoyoru97@outlook.com>")]
struct Args {
    ip_addr: String,
    #[clap(default_value_t = 8080)]
    port: u16,
}

/// 定义需要处理的文件类。
/// 
/// 为不同的文件类型适配不同的读写数据方式
pub enum FileType<'a> {
    Html,
    Json,
    Css,
    Js,
    Image(&'a str),
    Unknown,
}

impl<'a> FileType<'_> {
    /// 初始化文件格式
    pub fn init_file_type(file_type: &'a str) -> FileType {
        match file_type {
            "png" | "jpg" | "ico" | "jpeg" => FileType::Image(file_type),
            "html" | "htm"                 => FileType::Html,
            "json"                         => FileType::Json,
            "css"                          => FileType::Css,
            "js"                           => FileType::Js,
            _                              => FileType::Unknown,
        }
    }
    

    /// 获取 Content-Type (MIME Type)
    pub fn get_content_type(&self) -> &'a str {
        match &self {
            FileType::Html          => "text/html",
            FileType::Image("png")  => "image/png",
            FileType::Image("jpg")  => "image/jpg",
            FileType::Image("gif")  => "image/gif",
            FileType::Image("ico")  => "image/x-icon",
            FileType::Image("webp") => "image/webp",
            FileType::Js            => "application/javascript",
            FileType::Css           => "text/css",
            FileType::Json          => "application/json",
            _                       => panic!("No content type available"),
        }
    }

    /// 返回 image-rs 的输出格式
    pub fn get_img_output_fmt(&self) -> image::ImageOutputFormat {
        match &self {
            FileType::Image("png") => image::ImageOutputFormat::Png,
            FileType::Image("ico") => image::ImageOutputFormat::Ico,
            _                      => panic!("Image format illegal"),
        }
    }
}

/// 解析并处理客户端的 HTTP 请求
async fn read_http_request(mut stream: TcpStream, buffer: &[u8]) -> Result<(), Box<dyn Error>> {
    // TODO: 解析 HTTP Uri: https://docs.rs/http/0.1.18/http/uri/struct.Uri.html
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
                let file_type = FileType::init_file_type(
                                            file_vec
                                            .last()
                                            .unwrap_or(&"")
                                        );
                let path = format!("{}{}", file_root, sub_path);
                println!("{}", path);

                let mut bytes: Vec<u8> = Vec::new();
                let contents = match file_type {
                    FileType::Html => {
                        fs::read(path)
                            .await?
                    },
                    FileType::Image(_) => {
                        let img = ImageReader::open(path)?
                                .decode()?;
                        let scaled = img.resize(350, 200, FilterType::Triangle);
                        // TODO: 怎么支持异步读写操作？
                        scaled.write_to(&mut bytes, file_type.get_img_output_fmt())?;

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

/// 生成 HTTP Response
fn gen_http_response(contents: &[u8], file_type: FileType) -> Vec<u8> {
    let (status_code, text) = ("200", "OK");
    let status = format!("HTTP/1.1 {0} {1}{2}", status_code, text, CLDR);
    let server_name = format!("Server: Rust{0}", CLDR);
    let content_type = format!("Content-Type: {0};", file_type.get_content_type());
    println!("{}",  file_type.get_content_type());
    //let char_set = format!("charset=utf-8{0}", CLDR);
    let content_length = format!("Content-Length:{0}{1}\n", contents.len(), CLDR);

    let response = format!(
        "{0}{1}{2}{3}"
        , status, server_name, content_type, content_length
    );
    println!("{0}", response);

    [response.as_bytes(), contents].concat()
}

/// 处理已建立的 TCP 连接
async fn handle_tcp_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut message = String::new();
    BufReader::new(&mut stream)
                .read_line(&mut message)
                .await?;
    //println!("{}", message);
    
    read_http_request(stream, &message.as_bytes()).await?;
    
    Ok(())
}

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::Drain;

/// 异步的 HTTP 服务器
#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    // 输出日志到终端
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());

    // 读取命令行参数
    let args = Args::parse();
    let (server_addr, port) = (&args.ip_addr, args.port);
    println!("IP address {0} port {1}", &server_addr, port);

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