# Summary

- [序言](./序言.md)
- [环境搭建](./环境搭建.md)
- [从简单实现到模块化实现](./从简单实现到模块化实现/README.md)
  - [一个简单实现](./从简单实现到模块化实现/一个简单实现.md)
  - [拆分逻辑 & 让错误逐层传播]()
  - [加上测试和文档！]()
  - [再封装一下如何？]()
- [进程模型 vs 线程模型]()
- [每次都要找操作系统要线程？准备一堆干净可重用的线程到手头边]()
  - [建立线程池]()
- [线程不能一鱼多吃？I/O复用考虑一下]()
  - [阻塞I/O模型 vs 非阻塞I/O模型]()
  - [可复用的I/O，select & poll登场！]()
  - [Epoll]()
  - [信号驱动I/O模型]()
- [效率更高的异步I/O](./让服务器支持并发场景/README.md)
  - [Linux aio I/O模型]()
  - [加餐：还有其它I/O模型]()
- [系统地规划系统行为：采用线程模型]()
  - [Reactor线程模型]()
  - [Preactor线程模型]()

- [觉得不够（尽兴）？再写一个Socket库]()
