# 间章：从小打小闹到实战开发

在之前对单文件程序的一系列改进当中，相信大家对这次编写项目的目标有了一些实践上的认知。然而，在接下来的开发过程中，可不只是要写几个代码文件、应用一些模型，我们还更需要考虑程序代码之间的关系，将代码充分地模块化。

**我们先用实际开发用到的各类库搭一个原型，再从实际开发会遇到的一些问题穿针引线，讲一下各种 I/O 模型和线程模型。**



接下来会涉及到的：

- 用 protobuf 序列化/反序列化

  [rust-protobuf](https://github.com/stepancheg/rust-protobuf)  

- 用 Rust 写单独的 lib 模块

- 用 futures & tokio 支持异步操作

- 用 tracing 打造更完善的日志系统

- 用 [hyper](https://camposha.info/rust/rust-hyper-http/) 简化 HTTP 处理逻辑

[进化的 Http Server : 二 猴子都会写异步 使用 tokio ](https://rustcc.cn/article?id=56518c67-6dd1-431c-8361-8e9badd53b71)

[Reducing tail latencies with automatic cooperative task yielding](https://tokio.rs/blog/2020-04-preemption)



### 参考

- [RustChinaConf2020 精选 | Rust 异步与并发](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rust_async.html#rustchinaconf2020-精选--rust-异步与并发)
- [Tonic](https://github.com/hyperium/tonic)
- [[Rust Crate] log4rs Rust log 库](https://zhuanlan.zhihu.com/p/104921298)
- [Why Discord is switching from Go to Rust](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f)
- [用Rust做Web开发，时机成熟了](https://rustcc.cn/article?id=0e8e1b38-5180-4021-b6fe-e017eb8ff315)

- [Mocking HTTP Services in Rust](https://alexliesenfeld.com/mocking-http-services-in-rust)

