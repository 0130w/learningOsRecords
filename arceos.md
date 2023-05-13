# ArceOS学习

## 目录

- [ArceOS学习](#arceos学习)
  - [目录](#目录)
  - [组件化操作系统的初步探索](#组件化操作系统的初步探索)

## 组件化操作系统的初步探索

Linux也是组件化的，但是组件之间紧耦合，难以重用

组件化OS的设计目标
- 易于定制
  - 适应硬件与应用需求
  - 高安全、高性能、专用化
- 易于复用
- 易于开发

增加`crates`的数量,减少`modules`的数量

在`toml`文件里面的`libax`中增加一些`features`就能够为应用增加更多的功能

- 上层调用底层
    - ulib -> modules/crates
    - modules -> modules/crates
    - crates -> crates
    - 通过Cargo.toml指定依赖

- 底层调用上层?相互调用
  `crate_interface`

- 解决思路
  - `#[global_allocator]`
  - 大概就是在`crate`中只定义,然后不关心具体实现,在`modules`里面在具体实现`crate_interface`

应用自定义组件
- 文件系统/驱动程序种类繁多,不可能都添加进`crates/modules`中
- 在`app`层实现自定义组件
- 将底层默认的替换掉

组件化的思想可以进行独立的单元测试
- `cargo test`

基于`Rust`的语言和编译器的特性来构造一个OS,`modular OS`一种构建`OS`的方式

`unikernel`：一种`OS`的模式

`UniKernel OS/LibOS`

`Unikernel`的形式是单一应用的,然后也是单一地址空间的,且是单一特权级的
- 效率比较高
- 安全性有点问题,单一特权级

`Unikernel`一般会引入一个新的层`Hypervisor`用于隔离不同的`Unikernel`块

`Modular OS`
- 组件是什么? 什么是组件化的`OS`?
  组件是一种封装,外部只能通过公开接口进行通信
  在基于`Rust`语言实现组件的时候,组件就差不多是`Crate`

组件化`OS`,基于组件化来构建`OS` `Building Block`
- 思想:一切皆组件
- 建立一个工厂,通过工厂来生产`OS`
- 组件通常是预构建的,比较成熟

组件来源: OS共性与个性
- 发现: 无论是什么特性什么模式的`OS`,总有一部分是共通的,即有些工作步骤是必须的,且步骤之间存在一定的次序/依赖关系,这种**共性部分**封装为共性组件,可以为各类`OS`或某种`OS`的各类配置所复用

