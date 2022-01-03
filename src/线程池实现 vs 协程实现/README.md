# 线程池实现 vs 协程实现

看到这里是不是有些疑惑了？之前一直在讲异步 I/O 和协程，那其它很多实现里讲的线程池、epoll、reactor 和 preactor 模型又是怎么回事？





### 协程

- 协程能在等待 I/O的过程中（I/O 非阻塞）重复利用线程，减少系统内存开销和切换线程的开销，适用于 I/O 密集场景。
- 需要编程语言良好的原生异步 I/O 支持。

Rust 的许多高性能服务器都支持协程（Rust 的生态对协程的支持还是挺完善的）。这次我们就用 tokio 写一个支持协程的 HTTP 服务器。



### 参考

- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
- [tokio](https://tokio.rs/)

- [async_std](https://async.rs/)