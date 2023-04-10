# rCore学习笔记

## 目录

- [rCore学习笔记](#rcore学习笔记)
  - [目录](#目录)
  - [Chapter 0](#chapter-0)
  - [Chapter 1](#chapter-1)
    - [平台与目标三元组](#平台与目标三元组)

## Chapter 0

配置一下环境，然而我的ubuntu物理机上面的VSCode的Extension根本打不开，准备回到我忠实的Vim

## Chapter 1

这个图还挺好的，一眼就感觉挺清晰的

![app-software-stack](./image/app-software-stack.png)

### 平台与目标三元组

目标三元组是这样的一种概念：三元组的第一个元素描述了CPU的指令集，第二个元素描述了CPU的制造厂商，第三个元素描述了程序运行的操作系统(有时候还会带上运行时的标准库)，格式如下

```diff
arch-ventor-os
```

在Terminal中，我们可以通过如下的指令来看rust程序运行时的环境目标三元组

```shell
rust --version --verbose
```

然后我们要做的是把这个`Hello World!`程序移到一个裸机环境中，这就意味着我们不再有系统调用，也不再有标准库的支持，我们采取以下方式让程序在裸机环境下编译

```shell
cargo run --target riscv64gc-unknown-none-elf
```

由于我们的`println!`是`Rust`的标准库提供的，标准库又是基于系统调用实现的，这里自然就不能用了