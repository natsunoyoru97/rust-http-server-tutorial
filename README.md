# 用Rust实现一个HTTP服务器

**项目目前尚未启动，网络编程部分和HTTP服务器部分均未完成！！**

~~目前上传到github只是想警示一下喜欢鸽的自己，**你还有事要做赶紧干正事**~~

这篇教程是现学现卖，有什么知识理念上的错误和不足欢迎提出PR！

本教程由[mdBook](https://github.com/rust-lang/mdBook)制作，可以将代码仓库拷贝至本地运行电子书和示例代码：

- 本地运行电子书

```bash
  cd rust-http-server-tutorial
  mdbook serve
```

- 本地运行示例代码

```bash
  cd rust-http-server-tutorial
  cargo run --example <示例代码名称> -- [args..]
```

### 目前支持的示例

- simple_client

- simple_server

- simple_http_server

### 目前可以参考的性能测试结果

所有性能测试的结果均在``./loadtest_log``文件夹下。

目前只有 MacOS Catalina 下使用 npm 插件 loadtest 进行压力测试的结果。日后还会添加 Linux 平台的测试结果，并完善文档。

- simple_http_server


### 为什么用Rust？

#### 不正经的理由

因为实在不想看到每个HTTP服务器的实现都是C捆绑面条一样的代码。C确实最接近各个平台提供的网络套接字接口，但我不觉得这个优势足以让我舍弃**程序逻辑的清晰简洁和适当的模块封装，且在冗余代码少的情况下实现友好的用户交互**，鉴于网络服务器的逻辑比较复杂，面向过程的代码耦合度太高也难以迭代，所以C打咩（即答

既然C不行，那既兼容C又比C抽象表达能力更好的现代C++呢？C++也不是不行，但是我不太想和第三方库花太多时间纠缠，它的包管理也不咋友好（话说C++有模块和包管理的概念吗）🤔

有GC的语言就算了，Stop the World 的固有缺陷和一些[别的坑](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f)我并不想踩。Rust作为一门无GC、需要手动管理内存的系统编程语言，程序的性能理论上讲会比有GC的语言好一截，这次想啃一下硬骨头。

#### 正经的理由

我就是喜欢Rust，而且Rust在网络服务端的抽象能力和包管理还是可以的（叉腰。~~不然我要怎么从逻辑简洁的角度回答“为什么你不用go不用erlang不用kotlin”？~~



<!-- 可能你会觉得我是不是把“不正经的理由”和“正经的理由”给写反了，这还真不是。我要不是中意一门语言，哪会上手就干还顺带说一堆好话（用Rust写的缺点也是存在的）？如果我是C或者C++的布道者，我自然也能找到理由¯\_(ツ)_/¯ -->

