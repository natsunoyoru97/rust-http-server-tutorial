//! 使用 tokio 运行时搭建的异步 HTTP 服务器
//! tokio 运行时已经封装好了线程池、executor 和 poll/epoll
use clap::Parser;
use image::{
    //imageops::FilterType,
    io::Reader as ImageReader
};
use slog::{debug, info, trace, Logger};
use sloggers::{Config, LoggerConfig};
use std::{
    error::Error,
    str
};
use tokio::{
    fs,
    net::{TcpListener, TcpStream},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}
};

mod request;
use request::{ RequestHeader, FileType };

static CLDR: &str = "\r\n";

#[derive(Parser, Debug)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    ip_addr: String,
    #[clap(default_value_t = 8080)]
    port: u16,
}

/// 解析并处理客户端的 HTTP 请求
async fn read_http_request(mut stream: TcpStream, buffer: &[u8], logger: &Logger) -> Result<(), Box<dyn Error>> {
    trace!(logger, "-> async fn read_http_request");
    // TODO: 解析 HTTP Uri: https://docs.rs/http/0.1.18/http/uri/struct.Uri.html
    // 解析客户端发来的 HTTP 请求
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut request = httparse::Request::new(&mut headers);
    let res = request.parse(buffer).unwrap();

    if res.is_partial() {
        match request.path {
            Some(path) => {
                let req_header = RequestHeader::init_request("./public", path);
                let path = &req_header.get_full_path();
                debug!(logger, "{:?}", path);
                let file_type = &req_header.get_file_type();
                debug!(logger, "{:?}", file_type);

                let mut bytes: Vec<u8> = Vec::new();
                // FIXME: 目前不支持读取中文文件名
                let contents = match file_type {
                    FileType::Html | FileType::Css | FileType::Js => {
                        fs::read(path).await?
                    },
                    FileType::Image(_) => {
                        let img = ImageReader::open(path)?
                                .decode()?;
                        //let scaled = img.resize(350, 200, FilterType::Triangle);
                        // TODO: 怎么支持异步读写操作？
                        img.write_to(&mut bytes, file_type.get_img_output_fmt())?;

                        bytes
                    },
                    _ => bytes,
                };
                
                debug!(logger, "Ready to generate HTTP response");
                let response = gen_http_response(contents.as_slice(), *file_type, logger);
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
fn gen_http_response(contents: &[u8], file_type: FileType, logger: &Logger) -> Vec<u8> {
    trace!(logger, "-> fn gen_http_response");
    let (status_code, text) = ("200", "OK");
    let status = format!("HTTP/1.1 {0} {1}{2}", status_code, text, CLDR);
    let server_name = format!("Server: Rust{0}", CLDR);
    let content_type = format!("Content-Type: {0};", file_type.get_content_type());
    //let char_set = format!("charset=utf-8{0}", CLDR);
    let content_length = format!("Content-Length:{0}{1}\n", contents.len(), CLDR);

    let response = format!(
        "{0}{1}{2}{3}"
        , status, server_name, content_type, content_length
    );
    info!(logger, "{}", format!("{0}", response));

    [response.as_bytes(), contents].concat()
}

/// 处理已建立的 TCP 连接
async fn handle_tcp_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // 输出异步日志到终端
    // TODO: 有没有办法既能保留 move 操作，又不用重复初始化 logger？
    let config: LoggerConfig = serdeconv::from_toml_str(r#"
        type = "terminal"
        level = "debug"
        destination = "stderr"
        "#)?;
    let logger = config.build_logger()?;
    trace!(logger, "-> async fn handle_tcp_connection");

    let mut message = String::new();
    BufReader::new(&mut stream)
                .read_line(&mut message)
                .await?;
    read_http_request(stream, &message.as_bytes(), &logger).await?;
    
    Ok(())
}

/// 异步的 HTTP 服务器
#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    // 输出异步日志到终端
    let config: LoggerConfig = serdeconv::from_toml_str(r#"
        type = "terminal"
        level = "trace"
        destination = "stderr"
        "#)?;
    let logger = config.build_logger()?;
    info!(logger, "Logging ready!");

    // 读取命令行参数
    let args = Args::parse();
    let (server_addr, port) = (&args.ip_addr, args.port);
    info!(logger, "{}", format!("Start with IP address {0} port {1}", &server_addr, port));

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