use http::{Request, Response};

/// HTTP 封装
trait Http {
    fn new(pages: Vec<&'static str>) -> Self;

    /// 对不同路径的 HTTP 请求作出不同的响应
    fn response(req: Request<()>) -> http::Result<Response<()>>;
}

/// 用于测试的 HelloPage 实例
struct HelloPage {
    pages: Vec<&'static str>,
}

impl Http for HelloPage {
    fn new(pages: Vec<&'static str>) -> HelloPage {
        HelloPage {
            pages: pages,
        }
    }
}
