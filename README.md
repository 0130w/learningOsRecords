# learningOsRecords

- [Rust学习笔记](./rustnotes.md)
- [RISC-V学习笔记](./risc_vnotes.md)
- [rCore学习笔记](rCore.md)

---

## 目录

- [learningOsRecords](#learningosrecords)
  - [目录](#目录)
  - [每日学习记录](#每日学习记录)
    - [Day1](#day1)
    - [Day2](#day2)
    - [Day3](#day3)
    - [Day4](#day4)
    - [Day5](#day5)
    - [Day6](#day6)
    - [Day7](#day7)
    - [Day8](#day8)
    - [Day9](#day9)
    - [Day10](#day10)
    - [Day11](#day11)
    - [Day12](#day12)
    - [Day13](#day13)
    - [Day14](#day14)
    - [Day15](#day15)
    - [Day16](#day16)
    - [Day17](#day17)
    - [Day21](#day21)
    - [Day26](#day26)
    - [Day27](#day27)


---

## 每日学习记录

### Day1

> date: 2023.4.1

学习Rust语言，目前进度在Rust Course的包与模块章节，rustlings中的练习进度为51/94

### Day2

> date: 2023.4.2

学习Rust语言，目前进度在刚刚结束Rust Course的基础入门部分，rustlings中的练习进度为67/94

### Day3

> date: 2023.4.3

学习Rust语言，目前进度在Rust Course的结束了迭代器部分，rustlings中的练习进度为74/94

### Day4

> date: 2023.4.4

学习Rust语言，学习了一些高级进阶部分知识，rustlings中的练习进度为85/94

### Day5

> date: 2023.4.5

学习Rust语言，今天由于计网HW比较多所以留的时间不是很多，Rust Course的线程还是不太熟，*rustlings进度:Finished.*

### Day6

> date: 2023.4.6

休息，看了第一章的RISC-V的书

### Day7

> date: 2023.4.7

继续看RISC-V的书，整理了第一章内容

### Day8

> date: 2023.4.8

继续看RISC-V的书，笔记更新到了第二章的第8小节

### Day9

> date: 2023.4.9

复习了一下Rust语言，写了个minigrep上去，继续在看RISC-V的书

### Day10

> date: 2023.4.10

继续在看RISC-V的书，看到了第二章的第十节，感觉有些细节还挺麻烦的。顺便配置了一下rCore的实验环境

### Day11

> date: 2023.4.11

今天没有进度，复习了计算机网络，应对期中考

### Day12

> date: 2023.4.12

继续复习计网，基本搞定了

### Day13

> date: 2023.4.13

正在看那个手册，怎么说呢，感觉有点难，准备小溜一下视频讲解

### Day14

> date: 2023.4.14

忙了一会别的，准备周日的考试，顺便学了一下Verilog，做完了[HDLBits](https://hdlbits.01xz.net/wiki/Main_Page)的`Getting Start`和`Verilog Language`全部

### Day15

> date: 2023.4.15

在准备明天的考试

### Day16

> date: 2023.4.17

跟着写了一遍`lab1`，理解加深了(在操作的时候不小心把自己写的删掉了:(
边读`rCore-Tutorial`边查`risc-v`手册中，听了米老师的课

### Day17

> date: 2023.4.20

完成了第二阶段`ch3`

### Day21

> date: 2023.5.1

话说好像有两天的记录忘记保存了，没有存上来

这两天完成了第二阶段`ch4`和`ch5`，感觉还是挺有意思的，虽然还有很多地方其实不是很懂，实验的体会直接写在`reports`里面了
然后有两个很奇怪的地方:
- `ch4`那个`TaskInfo`的时间测例一直过不去，看了一下大概总是小了`20ms`就直接hack过去了，但其实不是特别懂为啥

- `ch5`关于虚拟地址测例是不是实际上没有考虑分页的，不用逐`byte`去翻译也能过?

### Day26

> date: 2023.5.6

完成了`ch6`,晚上看了`ch8`的部分内容

### Day27

> date: 2023.5.7

晕了,刚考完期中出考场就看到群消息,立马跑回去写,写到10点饭都没吃,还好昨晚把伪代码之类的都写了一下,也熟悉了一下`ch8`的代码框架

最难绷的是那个`sys_gettime`,少了这个也不报错的,就一直死锁卡在那,搞得我还以为是银行家算法写的有问题