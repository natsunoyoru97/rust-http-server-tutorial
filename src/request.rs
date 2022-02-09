

/// 请求头
#[derive(Debug, Clone)]
pub struct RequestHeader<'a> {
    root: &'a str,
    subpath: &'a str,
    file_type: FileType<'a>,
}

impl<'a, 'b> RequestHeader<'b> {
    /// 初始化 RequestHeader 类
    pub fn init_request(root_str: &'b str, path_str: &'b str) -> RequestHeader<'b> {
        let subpath_tmp = 
            match path_str {
                "/" => "/index.html",
                _ => path_str,
            };
        let file_vec = &subpath_tmp
                        .split('.')
                        .collect::<Vec<&str>>();

        RequestHeader {
            root: root_str,
            subpath: subpath_tmp,
            file_type: FileType::init_file_type(
                            file_vec
                                .last()
                                .unwrap_or(&"")
                        ),
        }
    }

    /// 获取 RequestHeader 的 file_type 成员
    pub fn get_file_type(&self) -> FileType {
        self.file_type
    }

    /// 获取 RequestHeader 的文件全路径
    pub fn get_full_path(&self) -> String {
        format!("{}{}", self.root, self.subpath)
    }
}

/// 定义需要处理的文件类。
/// 
/// 为不同的文件类型适配不同的读写数据方式
#[derive(Debug, Copy, Clone)]
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
        println!("init_file_type: {0}", file_type);
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
