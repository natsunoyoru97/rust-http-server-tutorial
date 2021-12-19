# 可复用的I/O，select & poll登场！

这时你可能会问，对应Unix C接口的select函数呢？



——官方库还真没有🤷‍♀️。

这时候tokio就闪亮登场啦！上回讲到用crate futures来搭建一个线程池，futures里有select!和poll!这两个宏，而tokio是对futures的又一层封装。

select!

poll!