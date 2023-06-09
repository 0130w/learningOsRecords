# Rust学习笔记

![Flower](image/DSC04196.jpg)

> Author: Ziqi Zhou

> 记录在学习Rust中遇到的一些有趣的东西,章节顺序暂时几乎按Rust Course来，在部分知识点只是对Rust Course的内容做了笔记

---

## 目录

- [Rust学习笔记](#rust学习笔记)
  - [目录](#目录)
  - [Rust基本介绍](#rust基本介绍)
    - [运行Rust项目](#运行rust项目)
    - [Cargo.toml 与 Cargo.lock](#cargotoml-与-cargolock)
  - [Rust基础入门](#rust基础入门)
    - [复合类型](#复合类型)
      - [枚举](#枚举)
      - [特征对象](#特征对象)
    - [流程控制](#流程控制)
    - [集合类型](#集合类型)
      - [Vector类型](#vector类型)
      - [Hashmap](#hashmap)
    - [生命周期](#生命周期)
      - [三条消除规则](#三条消除规则)
      - [声明生命周期的长度的语法](#声明生命周期的长度的语法)
      - [静态生命周期](#静态生命周期)
    - [返回值和错误处理](#返回值和错误处理)
      - [panic](#panic)
        - [被动触发](#被动触发)
        - [主动调用](#主动调用)
        - [栈展开](#栈展开)
        - [panic!的两种终止方式](#panic的两种终止方式)
        - [panic!之后](#panic之后)
      - [Result和?](#result和)
        - [Result的定义](#result的定义)
        - [unwrap和expect](#unwrap和expect)
        - [传播错误](#传播错误)
        - [宏?](#宏)
          - [一个常见错误](#一个常见错误)
          - [带返回值的main函数](#带返回值的main函数)
      - [Crate和Package](#crate和package)
        - [Crate](#crate)
        - [Package](#package)
          - [bin类型Package](#bin类型package)
          - [library类型Package](#library类型package)
          - [典型的Package结构](#典型的package结构)
        - [Modules](#modules)
          - [使用super引用模块](#使用super引用模块)
          - [使用self引用模块](#使用self引用模块)
          - [结构体和枚举的可见性](#结构体和枚举的可见性)
          - [模块与文件分离](#模块与文件分离)
        - [使用use引入模块以及受限可见性](#使用use引入模块以及受限可见性)
          - [引入模块还是引入函数](#引入模块还是引入函数)
          - [同名冲突问题](#同名冲突问题)
          - [引入项再导出](#引入项再导出)
          - [使用第三方包](#使用第三方包)
          - [使用{ }简化引入方式](#使用-简化引入方式)
          - [使用\*引入模块下的所有项](#使用引入模块下的所有项)
          - [受限可见性](#受限可见性)
      - [注释与文档](#注释与文档)
          - [注释种类](#注释种类)
        - [文档注释](#文档注释)
        - [Crate和Modules级别的注释](#crate和modules级别的注释)
        - [文档测试](#文档测试)
        - [文档注释中的代码跳转](#文档注释中的代码跳转)
          - [跳转到标准库/第三方库/自己的库](#跳转到标准库第三方库自己的库)
          - [跳转到同名项](#跳转到同名项)
        - [文档搜索别名](#文档搜索别名)
  - [Rust进阶学习](#rust进阶学习)
    - [生命周期](#生命周期-1)
      - [深入生命周期](#深入生命周期)
        - [生命周期检查](#生命周期检查)
        - [无界生命周期](#无界生命周期)
        - [生命周期约束](#生命周期约束)
        - [闭包的消除规则](#闭包的消除规则)
        - [NLL(Non-Lexical Lifetime)](#nllnon-lexical-lifetime)
        - [Reborrow](#reborrow)
        - [生命周期消除规则补充](#生命周期消除规则补充)
          - [impl块消除](#impl块消除)
      - [\&'static和T:'static](#static和tstatic)
          - [\&‘static](#static)
          - [T : 'static](#t--static)
          - [static到底针对谁](#static到底针对谁)
      - [函数式编程](#函数式编程)
        - [闭包](#闭包)
          - [结构体中的闭包](#结构体中的闭包)
          - [捕获作用域中的值](#捕获作用域中的值)
      - [迭代器(Iterator)](#迭代器iterator)
          - [惰性初始化](#惰性初始化)
          - [next方法](#next方法)
          - [IntoIterator特征](#intoiterator特征)
          - [into\_iter, iter, iter\_mut](#into_iter-iter-iter_mut)
          - [消费者与适配器](#消费者与适配器)
          - [实现Iterator特征](#实现iterator特征)
          - [enumerate方法](#enumerate方法)
          - [计算阶乘的例子](#计算阶乘的例子)
    - [深入类型](#深入类型)
      - [类型转换](#类型转换)
        - [强制类型转换](#强制类型转换)
        - [点操作符](#点操作符)
    - [智能指针](#智能指针)
      - [Box堆对象分配](#box堆对象分配)
          - [堆栈的性能](#堆栈的性能)
        - [Box的使用场景](#box的使用场景)
        - [Box::leak](#boxleak)
      - [Rc与Arc实现1vN所有权机制](#rc与arc实现1vn所有权机制)
        - [Rc\<T\>](#rct)
        - [Arc](#arc)
    - [多线程并发编程](#多线程并发编程)
      - [使用多线程](#使用多线程)
        - [创建线程](#创建线程)
        - [线程是如何结束的](#线程是如何结束的)
    - [宏](#宏-1)
        - [宏和函数的区别](#宏和函数的区别)
        - [声明式宏](#声明式宏)
        - [用过程宏为属性标记生成代码](#用过程宏为属性标记生成代码)
  - [常用工具链](#常用工具链)
    - [自动化测试](#自动化测试)
      - [编写测试以及控制执行](#编写测试以及控制执行)
  - [Appendix](#appendix)
    - [常用Crate](#常用crate)
      - [文件系统](#文件系统)
      - [I/O](#io)
    - [常用方法/函数](#常用方法函数)
    - [随手记](#随手记)


---

## Rust基本介绍


### 运行Rust项目

运行Rust项目的两种方式：
- `cargo run`
- 
    ```shell
    cargo build
    ./target/debug/proj_name
    ```
以上两种运行方式是等价的，并且可以发现我们此时是在debug模式下运行的，在这种情况下代码的编译速度会很快，但是运行速度会很慢，适合检查代码是否通过编译。

> debug模式下出现这种行为的原因在于此时编译器不会对代码做任何优化

当我们需要运行速度更快的代码时，我们可以切换到release模式
- `cargo run --release`
- 
    ```shell
    cargo build --release
    !Todo
    ```

当我们的项目变得很大的时候，运行`cargo run`和`cargo build`的速度都非常慢，此时我们可以通过`cargo check`来快速检查代码是否通过编译。

---

### Cargo.toml 与 Cargo.lock

这两个文件是cargo的核心文件，cargo的所有活动都基于这两个文件的配置，其中：
- Cargo.toml: cargo特有的项目数据描述文件，存储了项目的所有元配置信息
- Cargo.lock: cargo工具根据.toml文件生成的项目依赖详细清单

> **当我们上传Rust项目的时候，什么时候需要上传.lock文件呢？**
> - 如果我们的项目是一个可执行文件，那么需要上传.lock文件
> - 如果我们的项目是一个依赖库文件，那么不需要上传.lock文件

---

Rust语言原生支持UTF-8编码字符串

---

## Rust基础入门
  

### 复合类型

#### 枚举

我们来解析一下这句话：
> 只使用下划线本身，则并不会绑定值，因为 s 没有被移动进 _ 
> (Rust Course 2.6.4)

考虑如下的代码
```rust
fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(_) => {
                self.change_color(_);
            }
            _ => { }
        }
    }
```
这一段代码是不能够正常运行的，这正是由于在使用下划线本身的时候，并不会进行值的绑定，因此`self.change_color(_)`中的参数不能够被正确的传入，因此我们应该将下划线改成一些参数，如`color`

---

- Trait example 1

```Rust
use std::ops::Add;

// 为Point结构体派生debug特征
#[derive(Debug)]
struct Point<T : Add<T, Output = T>> {
    x : T,
    y : T,
}

// 这里是实现了一个方法
impl<T : Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p : Point<T>) -> Point<T> {
        Point { x: self.x + p.x, y: self.y + p.y }
    }
}

// 这里的T泛型参数到时候是用来传Point的
fn add<T : Add<T, Output = T>> (a: T, b: T) -> T{
    a + b
}

fn main(){
    let p1 = Point{x : 1.1f32, y : 1.1f32};
    let p2 = Point{x : 2.1f32, y : 2.1f32};
    println!("{:?}", add(p1, p2));
}
```

- Trait example 2

```rust
#![allow(dead_code)]

use::std::fmt;
use::std::fmt::{Display};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name : String,
    data : Vec<u8>,
    state : FileState,
}

// 使用format！ macro的时候只需要实现Display trait即可

impl Display for FileState {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(s : &str) -> File{
        File{
            name : String::from(s),
            data : Vec::new(),
            state : FileState::Closed,
        }
    }
}

fn main () {
    let f6 = File::new("f6.txt");
    
    // :?将变量按照其实现的Debug trait的方式输出一个字符串
    println!("{:?}", f6);
    println!("{}", f6);
}
```

#### 特征对象
当实现的函数/方法试图将返回类型设为实现了某一个特征的类型时，如果这些实现了某一个特征的类型的类型不同，编译器不允许返回，我们考虑如何解决这个问题。

1. 利用枚举类型
2. 利用特征对象

- example 1 
```rust
pub struct Screen {
    pub components : Vec<Box<dyn Draw>>,
}
```

如果使用泛型，我们可以用如下的写法，但这就会导致Vec中的所有都是同一个类型的数据，而上面使用特征对象的写法就不需要Vec中的都是同一类型的数据
```rust
pub struct Screen<T : Draw> {
        pub components : Vec<T>,
}
```

注意，当我们需要指明函数参数是实现了某种特征的时候，需要采用如下的写法

```rust
fn function(a : impl trait_a, b : impl trait_b) -> ... {}
```

如果需要表示引入多种特征都可以调用该函数

```rust
fn function(a : impl trait_a + trait_b) -> ... {}
```

也可以引入泛型后用`where`约束

```rust
fn function<T>(a : T) where T : trait_a + trait_b -> ... {};
```

### 流程控制

循环标签

```rust
'label : loop {
  'qwq : loop {
    break 'label;
  }
}
```
这玩意可以直接跳出最外面一层[感觉在什么语言见过]

### 集合类型

#### Vector类型

- 创建Vec类型的方法
    - `Vec::new()`
    - `Vec::with_capacity(capacity)`
    - `vec![...]`
- 修改Vec类型数组方法
  - `v.push(_)`

- 从Vec类型数组中读取数据
  - `&v[2] : 返回一个索引`
  - `v.get(2) : 返回一个Option<T>类型,需要使用match解构`
  
- 遍历Vec中的元素
  - ```rust
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }
    ```

- 存储不同类型的数据在数组中
  - 使用enum
  - 使用特征对象

> **为什么需要两种读取数据的方式？**
> - 在确保数组不会越界的情况下，我们可以使用第一种
> - 但在输入数据由用户确定的情况下，我们需要选择第二种，此时get在没有值的时候会返回None，比较安全

#### Hashmap

> 使用之前从标准库中导入该类型
```rust
use std::collections::HashMap;
```

- 创建HashMap
  - `let m = HashMap::new()`
  - `let m = HashMap::with_capacity(capacity)`

- 向HashMap中插入元素
  - `m.insert(K, V);` 

  - 如何将`Vec<(String, i32)>`类型转化为`HashMap<String, i32>`
    - 遍历列表做插入
    - 先将Vec转化成一个迭代器再使用collect方法收集起来
      ```rust
      let map : HashMap<_, _> = vec_name.into_iter.collect();
      ```

> collect是一个泛型方法，可以通过所需的目标类型推断出不同的实现，为了实现这一点，Rust使用了一个名为`FromIterator`的特征

- HashMap的所有权规则如下：
  - 如果Key实现了Copy特征，则该类型被复制进HashMap中
  - 如果Key类型没有实现Copy特征，那么该类型的所有权将被转移

> - 如果将引用放入HashMap中，需要确保该引用的生命周期至少与HashMap的生命周期一样长
> - 我们可以使用`std::mem::drop(name)`来手动将一个引用对应的内容从内存中删除

- 查询HashMap中的元素
  - get方法
    - ```rust
        let a = key;
        let b : Option<&type value> = map.get(&a);  //借用
      ```
    - 如何获得value类型的返回值?
      - ```rust
        let b : type value = map.get(&a).copied().unwrap_or(-1);
        ```
      - 遍历HashMap的KV对
  
    - 如何更新HashMap中的值？
      - 覆盖掉已有的值(假定HashMap中已存在`key = a`)
        - `let value = map.insert("a", 20);`
      - 若不存在某个`key`则插入，若存在就不管
        - `let value = map.entry("a").or_insert(20)`
      - 若不存在某个`key`则插入，若存在则更新
        - ```rust
            let mut map = HashMap::new();
            let value = map.entry("a").or_insert(20);
            *value = 20;
            ```

### 生命周期

```rust
'a 这种写法意味着生命周期要大于等于a
```
**函数或者方法中，参数的生命周期被称为`输入生命周期`，返回值的生命周期被称为`输出生命周期`**

#### 三条消除规则

编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期，具体来说：

1. 每一个引用参数都会获得一个生命周期
2. 当输入生命周期只有一个的时候，该生命周期会被赋给所有的输出生命周期
3. 当输入生命周期有多个，且参数中有一个是`&self`或是`& mut self`的时候则`&self`或`&mut self`的生命周期被赋给所有的输出生命周期

#### 声明生命周期的长度的语法
 ```rust
 'a : 'b
 ```
 这种写法表示限制了`a`的生命周期要大于等于`b`

#### 静态生命周期

```rust
'static
```
这种生命周期的声明方式可以标识某引用的声明周期可以与程序一样的久

### 返回值和错误处理

Rust中的错误主要分为两类：
- **可恢复错误**:通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误，这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
- **不可恢复错误**:全局性/系统性错误，如数组访问越界这样的错误

针对于以上两种错误，Rust语言中给出了两种不同的错误处理手段：
- **`Result<T, E>`**:用于处理可恢复错误
- **`panic`**:用于处理不可恢复错误

#### panic

##### 被动触发
 
如访问数组越界，Rust会直接抛出一个异常，这是Rust给我们的一种保护机制

##### 主动调用

Rust提供了一个panic!宏，当调用执行该宏的时候，打印出传入的错误信息，并展开**报错点往前的函数调用堆栈**，最后退出程序

```rust
fn main() {
  panic!("You failed!");
}
```

然后在终端上我们可以得到相应的输出信息

```shell
thread 'main' panicked at 'You failed!', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

##### 栈展开

我们可以通过终端给我们的信息，在运行的命令之前加上一个环境变量用于获取更详细的栈展开(栈回溯)信息

```shell
RUST_BACKTRACE=1 cargo run
```

> 注意需要获取栈展开信息是需要让程序在**debug**模式下运行的

```shell
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8460ca823e8367a30dda430efda790588b8c84d3/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/8460ca823e8367a30dda430efda790588b8c84d3/library/core/src/panicking.rs:64:14
   2: complex_num::main
             at ./src/main.rs:2:5
   3: core::ops::function::FnOnce::call_once
             at /rustc/8460ca823e8367a30dda430efda790588b8c84d3/library/core/src/ops/function.rs:250:5
```

以上就是我们的栈展开的信息了，最近的函数调用排在**最下面**

##### panic!的两种终止方式

- 栈展开模式
  - 默认的`panic!`的终止模式，相当于在.toml文件中设置为`panic = 'unwind'`
  - 在这种方式下，`panic!`被触发后Rust会回溯调用栈，依次执行每个栈帧的析构函数，一直查找到程序开头然后退出
  - 在这种模式下由于`panic!`之后还执行了栈展开的操作，因此程序编译出的二进制文件的大小可能会大一些

- 直接终止模式
  - 需要指定`panic = 'abort'`
  - 在这种情况下当程序调用了`panic!`之后程序直接退出，不做多余的操作

##### panic!之后

线程`panic!`之后，如果是在`main`线程中`panic!`则程序终止，如果不是在`main`中，那么子线程会终止，但是不会让整个程序终止

#### Result和?

##### Result的定义
```rust
enum Result {
  Ok(T),
  Err(E),
}
```

> 如何去查询一个函数的返回值类型?
> - RTFM
> - VSCode + rust-analyzer
> - 标记一个错误的类型，让编译器告诉你


##### unwrap和expect

当我们不想去`match`，而是希望一旦`Err`就直接`panic!`，我们可以这样做

```rust
let f = File::open("hello.txt").unwrap();
```
在上面这个例子里，`File::open("hello.txt)`会返回一个`Result<T, E>`类型，然后`unwrap`会去处理这个`Result`类型的返回值，如果错误就直接`panic!`

我们也可以用`expect`方法来自定义一些匹配错误时的信息
```rust
let f = File::open("hello.txt").expect("qwq");
```
这个和`unwrap`方法基本一样，只不过改了一下失败的时候到输出信息

##### 传播错误

我们的错误不一定当场处理，有可能需要往上传播

##### 宏?

`?`是一个宏，可以帮助我们直接实现往上传播

> `?`可以用于Result和Option的返回

###### 一个常见错误

`?`只能返回错误/`None`的值，在匹配成功的时候需要一个变量来承载，例如

```rust
fn first(arr : &[i32]) -> Option<&i32> {
  arr.get(0)?
}
```

这种写法是错误的，正确如下

```rust
fn first(arr : &[i32]) -> Option<&i32> {
  let v = arr.get(0)?
  Some(v) //注意这里不是v, v此时得到的是&i32的类型值
}
```

###### 带返回值的main函数

> main函数有多种返回值，这是因为实现了std::process::Termination特征

> https://doc.rust-lang.org/std/process/trait.Termination.html

#### Crate和Package

理清下面几个概念：

1. Package:用于构建、测试、分享Crate
2. Workspace:对于大型Package,可以进一步将多个Crate联合在一起构成Workspace
3. Crate:一个由多个Module组成的**树形结构**，可以作为第三方library进行分发，也可以生成可执行文件进行运行
4. Module:可以一个文件多个Module，也可以一个文件一个Module，Module可以被认为是真实Package中的代码组织单元

##### Crate

一个独立的可编译单元，编译后会形成一个可执行文件或一个库

一个Crate会将相关联的功能打包在一起，使得该功能可以很方便的在多个Package中分享，在使用时，我们只需要将Crate引入到当前Package的作用域中，就可以使用该Crate的功能

同一个Crate中不能有同名的类型，但不同的Crate中可以有不同的类型：
```rust
use a::rand;
use b::rand;
```
例如在上面这个例子中，我们在Crate`a`和Crate`b`之中都定义了`rand` 但是对于编译器而言使用`rand`时并不会产生混淆，因为我们是通过`a::rand`和`b::rand`来访问的

Crate的根文件标识了编译该Crate的时候的入口，一个Crate可以包含有多个文件

##### Package

包含独立的.toml文件以及因为功能性被组织在一起的一个或多个Crate

一个Package中只能包含一个library类型的Crate，但是可以包含多个bin类型的Crate

###### bin类型Package

Rust中有一个惯例，对于bin类型Package而言，`src/main.rs`是二进制Crate，该二进制Crate的CrateName和**所属的Package的名字**相同，所有的代码执行都从该文件的`fn main()`开始

###### library类型Package

library类型的Package只能作为第三方库被其他的Package引用，而不能独立运行

Cargo知道，如果Package包含有`src/lib.rs`那么意味着它包含一个库类型的同名Crate，该Crate为`lib.rs`

###### 典型的Package结构

```shell
my_rust/
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs

```

> 在上面这个例子中：
> - `main1.rs`, `main2.rs`都被视为一个独立的crate
> - `main.rs`是bin类型crate的入口点，`lib.rs`是lib类型crate的入口点，他们会编译最终得到一个名为my_rust的crate，`main.rs`和`lib.rs`都是my_rust这个crate的root file
> - tests下面的`some_integration_tests.rs`被认为是一个独立的crate，benches和examples同理

##### Modules

- 使用`mod`来创建新模块，后紧跟模块名
- 模块可以嵌套
- 模块中可以定义各种Rust类型

可以优先考虑使用绝对引用，因为调用的地方与定义的地方往往是分离的，定义的地方不会经常变动

在Rust中，默认的定义是private的，且子类型可以访问父类型但父类型不能随意访问子类型
```rust
mod front_of_house {
  mod hosting{
    fn add_to_waitlist() {}
  }
}

pub fn eat_at_restaurant() {
  crate::front_of_house::hosting::add_to_waitlist();
}
```
上面这种写法是错误的，原因在于父模块`front_of_house`不能去直接访问子模块`hosting`，为了正确运行，应该改为

```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {} //注意 函数也要改
  }
}
```

###### 使用super引用模块

从父模块开始的引用方式

> 有点像文件中的`../`

###### 使用self引用模块

从自身模块开始的引用方式

> 有点像文件中的`./`

###### 结构体和枚举的可见性

- 将枚举设为`pub`其所有字段也自动为`pub`
- 将结构体设为`pub`其所有字段依然是`private`

###### 模块与文件分离

如果需要将文件夹作为一个模块，我们需要指定要暴露哪些子模块

##### 使用use引入模块以及受限可见性

###### 引入模块还是引入函数

一般而言引入函数会看起来更简洁，但是如果有以下情况建议引入模块

- 需要引入统一模块的多个函数
- 作用域中存在同名的函数

> 一般而言建议是最细粒度的引入，出现问题再引入模块

###### 同名冲突问题

自然的解决办法是直接引入模块

另一种办法是别名引入，例如下面的`fmt`和`io`模块都包含了`Result`类型，我们可以将其中一个换一个名字

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

###### 引入项再导出

当外部模块项引入到当前模块时，在当前模块中该模块项被自动设为`private`，如果希望别的模块要从当前模块引用该模块项，可以在该模块项引入前面再加上`pub`

###### 使用第三方包

1. 修改.toml文件，修改`[dependencies]`项(rust-analyzer会自动拉取该库)
2. 在代码中使用

> 可以在Rust社区中lib.rs查找包，然后下载得在crates.io下载

###### 使用{ }简化引入方式

**例1**:
```rust
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::io;
```
改为
```rust
use std::collections::{HashMap, BTreeMap, HashSet};
use std::{io, cmp::Ordering};
```

**例2**:
```rust
use std::io;
use std::io::Write;
```
改为(注意这里的`self`)
```rust
use std::io::{self, Write};
```

###### 使用*引入模块下的所有项

```rust
use::std::collections::*;
```
但需要注意，引入的东西很可能和自己的在作用域中已经定义的东西重名，此时对于编译器而言，**本地定义的东西的优先级高于引入的**

###### 受限可见性

如果我们希望某个东西在当前的crate内是可见的，我们可以如下定义
```rust
pub(crate) item;
```
如果我们希望某个东西在整个更大的crate中可以定义，我们有两种选择方式
- 在crate root中定义module，此时父模块的项相对于子模块是可见的
- 在子模块中定义后引入到父模块

一个例子:

```rust
mod delicious_snacks {
    // TODO: Fix these use statements
    use self::fruits::PEAR as fruits;
    use self::veggies::CUCUMBER as veggies;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits,
        delicious_snacks::veggies
    );
}
```
这段代码不能够正常通过编译，原因在于我们尝试在外部`crate`中去访问子`module delicious_snacks`中的项,而里面的项是`private`的，为修改，应加入`pub`

**限制性语法**

```rust
pub //可见性无限制
pub(crate)  //在当前包内可见
pub(self) //在当前模块可见
pub(super)  //在父模块可见
pub(in <path>)  //在某个路径的模块可见，其中<path>必须是父模块/祖先模块
```

> 在同一个模块中一个**模块项**可以访问其他的**模块项**，即使是私有的

#### 注释与文档

###### 注释种类

在Rust中，注释共分为以下的三类
1. 代码注释：用于说明某一块代码的功能，读者为同一项目的开发者
2. 文档注释：支持Markdown，对项目描述、公共API等用户关心功能进行介绍，同时提供实例代码，读者为想要了解项目的人
3. 包和模块注释，算是文档注释中的一种主要说明包和模块的功能

**甚至能在文档注释中编写测试用例**
> rustlings中的 I AM NOT DONE不知道算不算

##### 文档注释

> 我去，这也太好用了

**文档注释要位于lib类型的crate下面**

**文档行注释** `///`

**文档段注释** `/** ... */`

利用`cargo doc`可以直接生成HTML文件放在`target/doc`目录下

我们也可以直接用`cargo doc --open`生成HTML文件并在浏览器打开

常用的文档标题(*只是建议*)

- **Example**
- **Panic** : 函数可能会出现的异常状况
- **Errors** : 描述可能会出现的错误以及什么情况会导致错误
- **Safety** : 如果函数使用了`unsafe`的代码，调用者就需要注意一些使用条件以确保`unsafe`块的正常工作

##### Crate和Modules级别的注释

**行注释** `//!`

**段注释** `/*! ... */`

##### 文档测试

文档注释可以直接用于测试，在测试时，可能需要完整路径来调用函数，这是因为测试是在另外一个独立的线程中进行的

如果允许一个文档中的测试用例`panic`,可以如下写
```rust

///```rust, should_panic
/// Code
///```
```

当我们希望保留文档的测试功能，又希望测试用例的内容从文档中隐藏，我们可以在前面加上`#`

```rust
/// # ```rust
/// # Code
/// # ```
```

##### 文档注释中的代码跳转

###### 跳转到标准库/第三方库/自己的库

```rust
/// [`Option`]
```
这个可以直接跳转到标准库的枚举类型

也可以使用完整路径的方式进行跳转

```rust
/// [`std::future::Future`]
/// [`crate::MySpecialFormatter`]
```

###### 跳转到同名项

```rust
/// 跳转到结构体 ['Foo'](struct@Foo)
/// 跳转到同名函数 ['Foo'](fn@foo)
/// 跳转到同名宏 ['foo!']
```

##### 文档搜索别名

写法1:
```rust
#[doc(alias = "x")]
#[doc(alias = "qwq")]
pub struct owo{}
```

写法2:
```rust
#[doc(alias("x", "qwq"))]
pub struct owo{}
```

---

## Rust进阶学习

### 生命周期

#### 深入生命周期

##### 生命周期检查

例1:分析以下的代码，模拟编译器对代码进行生命周期标注

```rust
#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
        &'a *self
    }
    fn share<'a>(&'a self) {}
}

fn main() {
  'b : {
    let mut foo = Foo;
    'c : {
        let loan = foo.mutate_and_share();
        'd : {
            foo.share();
              }
        println!("{:?}", loan);
    }
  }
}
```

这段代码不能编译的原因是`foo.share()`的不可变引用与`foo.mutate_and_share()`可变引用同时存在，由规则3可知，`loan`的生命周期应该与`foo`可变引用的生命周期一致，因此`foo`的可变引用在`c`的全过程都存在，因此`d`过程中使用`foo`的不可变引用会报错

> 按照这个分析，只要我们去掉println!("{:?}", loan);就能够成功运行
> 这个的原因其实在于我们的规则3，这个规则说不定并不是一个特别好的规则，只是一个相对而言比较粗糙的实现

例2:分析以下的代码，模拟编译器对代码进行生命周期标注

```rust
#![allow(unused)]
fn main() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        match map.get_mut(&key) {
            Some(value) => value,
            None => {
                map.insert(key.clone(), V::default());
                map.get_mut(&key).unwrap()
            }
        }
    }
}
```

这个感觉就比较难受，编译器知道`match map.get_mut`时创建了一个可变借用，但是它不知道在`None`的时候这个可变引用可以被释放掉，因此给这个可变引用创建了一个很大的作用域，从而导致了`map.insert(key.clone(), V::default())`这里无法创建一个可变引用

##### 无界生命周期

`unsafe`代码经常会凭空产生引用，而一个引用必须会有生命周期，这种生命周期被称为无界声明周期 _(unbound lifetime)_

无界生命周期经常发生在解引用裸指针 _(raw pointer)_ 的时候

```rust
fn f<'a, T>(x : *const T) -> &'a T {
  unsafe {
    &*x
  }
}
```

`'a`这种无界生命周期没有受到任何约束，想要多大就有多大，在上面的例子中，比如T是一个引用，表示为`&'b T`，此时`'a`为`'unbounded`，那么`&'unbounded &'b T`会被视为`&'b &'b T`而通过编译检查

> 我们需要尽可能避免无界生命周期，因为这很可能会导致悬垂引用与内存泄漏问题，编译器无法去释放内存

##### 生命周期约束

> **被引用者的生命周期必须要大于等于引用者**

- `'a : 'b`
  表示生命周期`'a`必须长于`'b`

- `T : 'a`
  表示类型`T`的生命周期要长于`a`，这在Rust1.30后会自动推导

> 在进行类型转换的时候，也只能够将具有更长生命周期的类型转换为生命周期较短的类型

##### 闭包的消除规则

对于函数体而言，它的消除规则之所以能生效是因为其生命周期完全体现在签名的引用类型上，但闭包与函数不一样

```rust
let closure_slision = |x : &i32| -> &i32 {x};
```

这段代码不能够正常通过编译

> **用Fn特征解决闭包生命周期**

##### NLL(Non-Lexical Lifetime)

简单来说就是Rust1.31以前的版本的编译器的规则都是(Lexical Lifetime):引用的生命周期从借用开始，一直持续到作用域结束

在Rust1.31以后，引入了NLL规则：引用的声明周期从借用开始，一直持续到最后一次使用的时候

> https://github.com/rust-lang/polonius

这个我拿一段stackoverflow上的代码例子

```rust
fn main() {
  let v = vec![1, 2, 3];
  let score = &v[0];
  v.push(4);
}
```

在以前这段代码是不能通过编译的，原因在于score对v的引用一直持续到花括号结束，现在可以了:)

##### Reborrow

```rust
fn main() {
  let mut p = Point {x : 0, y : 0};
  let r = &mut p;
  // reborrow
  let rr : &Point = &*r;
  println!("{}", rr); // 由NLL,rr作用域结束
  r.move_to(10, 10);
  println!("{}", r);
}
```
这里,`rr`是对`r`的`reborrow`，但不能在`rr`的生命周期内再次使用`r`

##### 生命周期消除规则补充

###### impl块消除

```rust
impl <'a> Reader for BufReader<'a> {
  // not used 'a
}
```

以上的特征中没有用到那个声明周期，于是我们可以改写成如下形式

```rust
impl Reader for BufReader<'_> {
  // methods
}
```

#### &'static和T:'static

字符串字面值具有`'static`的生命周期

```rust
`&'static str
```
除此之外特征对象也具有`'static`的生命周期

另一种`'static`的用法是`T : 'static`

###### &‘static

对于`&'static`而言，生命周期仅针对于该**引用**，而不针对于**持有引用的变量**

持有该引用的变量依然受到`NLL`规则的限制

###### T : 'static

- case 1 :
  ```rust
  fn print_it<T: Debug + 'static>( input: T) {
      println!( "'static value passed in is: {:?}",input);
  }

  fn main() {
      let i = 5;
      print_it(&i);
  } // i drop
  ```
考虑上面这段代码，`i`在花括号末位被drop，但此时程序还没有结束，因此对`T`约束的`static`无法满足，将程序改成如下形式

```rust
fn print_it<T: Deubg + 'static>(input: &T) {
  println!("static value passed in is: {:?})", input);
}

fn main() {
  let i = 5;
  print_it(&i);
}
```
这段代码不会报错，原因在于我们输入的是`&T`，根本没用到`T`，因此编译器不会去检查`T`的特征是否满足，只会去看`&T`的生命周期

###### static到底针对谁

针对于**引用指向的数据**而不是引用，引用指向的数据被写进了二进制包里面

#### 函数式编程

函数式特性:
- 闭包(Closure)
- 迭代器 (Iterator)
- 模式匹配
- 枚举

##### 闭包

闭包是一种匿名函数，可以**赋值给变量**也可以**作为参数传递给函数**，不同于函数的是，它允许捕获调用者作用域中的值

> 注意是闭包赋值给变量，因此变量就是闭包函数，而不是将闭包的值赋值给变量，因此下面的做法是可以的
```rust
  let fun = || {...}; //此时fun是一个闭包
  fun(parm1, parm2, ...)  //返回值为闭包最后的表达式值
```

闭包的形式定义如下：

- ```rust
  |parm1, parm2, ...| {
      sentence 1;
      sentence 2;
      return_value
  }
  ```

当只有一个返回表达式的时候可以简化为

- `|parm1, ...| return_value
  
###### 闭包的使用

闭包的类型推导不是泛型，当编译器推导出一种类型后就会一直使用该类型

```rust
let example_closure = |x| x;

let s = example_closure(String::from("qwq"));
let n = example_closure(1);
```

上面这段代码是不行的，理由是编译器从第一次调用`example_closure`的时候推导出了闭包中`|x|`的类型为`String`，因此下面一行传入的参数就有问题了

> 有没有什么办法能够让闭包接受一个泛型呢？

###### 结构体中的闭包

> 实现一个简易的缓存，功能是获取一个值并将其存起来

```rust
struct Cacher<T>
where T : Fn(u32) -> u32,
{
  query : T,
  value : Option<u32>,
}
```

`query`是一个闭包，该闭包实现了特征`T`，然后这个特征是`Fn(u32)-> u32`，这个是std库里面说了为闭包自动实现的

**思考：如何实现让闭包返回值为一个泛型**

###### 捕获作用域中的值

#### 迭代器(Iterator)

迭代器和For循环的区别最主要的地方在于**是否通过索引来访问集合**

在Rust中，数组并不是一个迭代器，但Rust自动为数组实现了`IntoIterator`特征，在`for`中，Rust通过`for`语法糖，将实现了该特征的数组自动转换成了一个迭代器，因此我们可以

```rust
let v = [1, 2, 3, 4];
for i in v {
  println!("{}", i);
}
```

类似地，我们也可以

```rust
for i in 1..=10 {
  println!("{}", i);
}
```

实现了`IntoIterator`特征的类型具有一个`into_iter`方法，我们可以将这些类型显式地转化为迭代器

```rust
for i in v.into_iter() {
  println!("{}", i);
}
```

###### 惰性初始化

在Rust中，迭代器是惰性的，在仅创建而不初始化的时候，不会发生任何事

###### next方法

迭代器不通过索引访问元素，它的遍历迭代器的方法是怎么样的?

考虑如下的一个特征

```rust
pub trait Iterator {
  type Item;  // 关联类型Item，用于代替遍历的值的类型
  // 例如在数组中，Item = i32这样(自动推断)
  fn next(&mut self) -> Option<Self::Item>;

  // 省略其余有默认实现方式的方法
}
```

某个类型需要转化为迭代器，需要实现`IntoInterator`特征，而一个迭代器之所以是迭代器，是因为它实现了`Iterator`特征，因此`for`访问迭代器中的元素可以通过`next`方法来实现

**手动迭代的时候需要将迭代器标明为可变的，在使用`for`进行迭代的时候不需要做这件事，因为`for`会帮我们完成这件事**

`next`方法对迭代器的是**消耗性**的，最终迭代器中是没有任何元素的

例: 模拟了`for`循环
```rust
fn main () {
    let values = [1, 2, 3];

    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => {
                    println!("{}", x);
                }
                None => {
                    break;
                }
            }
        },
    };
    result
}
```

###### IntoIterator特征

难绷，迭代器自己也实现了`IntoIterator`特征

###### into_iter, iter, iter_mut

- `into_iter`会转移所有权
  返回值为`Some(T)/None`
- `iter`是不可变引用
  返回值为`Some(&T)/None`
- `iter_mut`是可变引用
  返回值为`Some(&mut T)/ None`

###### 消费者与适配器

消费者是迭代器上的方法，它会消费掉迭代器中的元素，并返回该元素的值，这些所有的消费者都需要用`next`方法来消费元素

- 消费者适配器
  只要迭代器中的某个方法`A`在其内部调用了`next`方法，那么`A`就被称为消费性适配器，由于`next`方法的调用会消耗迭代器中的元素，所以调用`A`方法会消耗迭代器中的元素

  > 例子：`sum`方法，拿走迭代器的所有权，并不断调用`next`方法对迭代器中的元素做加法

  **消费掉一个迭代器，返回一个值**

- 迭代器适配器
  迭代器适配器是惰性的，在单独作用的时候不产生任何的行为，它需要一个消费者适配器来收尾
  > 例子：`map`方法是一个迭代器适配器
  ```rust
  let v1 = vec![1, 2, 3];
  let v2 : Vec<_> = v1.iter().map(|x| x + 1).collect();
  ```
  这里的`collect`方法就是一个消费者适配器

- `collect`
  
  前面已经看到`collect`方法是一个消费者适配器，使用这个方法可以将迭代器中的元素收集起来转化为一个类型

  > 注意我们需要显式地告诉编译器我们希望用`collect`转化的类型，这是因为`collect`非常强大，可以转化为多种不同的类型，编译器不知道我们希望转化成什么类型

  例：用`collect`方法收集一个`HashMap`
  ```rust
  let names = ["sunface", "sunfei"];
  let age = [18, 18];
  let folks : HashMap<_, _> = names.iter().zip(age.iter()).collect();

  println!("{:?}",folks);
  ```

###### 实现Iterator特征

其他的集合类型也可以像数组一样创建迭代器，例如`HashMap`，我们也可以自己创建自己的迭代器，只需要我们自己定义一个类型，然后去实现为这个类型去实现一个`Iterator`的特征即可

例：接下来我们创建`Counter`这个迭代器

```rust
struct Counter {
  count : u32
}
```
然后为`Counter`实现一个关联函数`new`，用于创建新的`Counter`实例

```rust
impl Counter {
  fn new () -> Counter {
    Counter{count : 0}
  }
}
```

有了这个，我们就可以为`Counter`实现`Iterator`特征了

```rust
impl Iterator for Counter {
  type Item : u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else None
  }
}
```

这样我们就得到了一个迭代器

这个迭代器的其他方法都具有默认实现[基于`next`方法]，因此无需一个一个手动实现

###### enumerate方法

`enumerate`方法是一个迭代器适配器，针对于`for`循环我们可以采用`enumerate`方法获取迭代器中元素的索引

```rust
let v = vec![1, 2, 3];
for (i, v) in v.iter().enumerate() {
  println!("第{}个值是{}",i, v);
}
```

`enumerate`方法会返回一个迭代器，每个迭代器中的元素是一个`tuple`，为`(index, value)`

迭代器的性能比索引访问的效率更高，迭代器是`Rust`的零成本抽象之一

> 零开销原则：What you don't use, you don't pay for. And further: What you use, you couldn't hand code any better : )
> <div style = "text-align:right">----Bjarne Stroustrup</div>

###### 计算阶乘的例子

```rust
pub fn factorial(num : u64) -> u64 {
  match num {
    0 => 1,
    value => {
      let mut u = 1;
      for i in 1..=value {
        u *=i;
      }
      u
    }
  }
}
```

然而这个方法早已被rustlings的设计者看破(不是)，并给出了指导意见，可以去看看`fold`方法或者`rfold`方法，于是我写出了下面很丑的代码:P
```rust
match num {
        0 => 1,
        value => {
            (1..=value).map(|x| x).collect::<Vec<_>>().iter().fold(1u64, |mul, elem| mul * elem)
        }
    }
```

然后熟悉了之后就会发现很多操作都可以一行搞定:P

### 深入类型

#### 类型转换

Rust中不同类型的变量不能够做比较，在做比较的时候，我们需要把**较小的类型**转化为**较大的类型**，将*较大的类型*转化为*较小的类型*是错误的

可以采用以下的代码来看一个类型的最大值

```rust
let a = u16::MAX;
```

> 类似可以查看最小的之类的

常用的转换形式

```rust
let a = 3.1 as i8;
```

> Rust中的浮点数转换为整型是向下取整的

> Rust中有两种指针类型，一种是safe的引用，一种是unsafe的裸指针

将内存地址转化为指针(裸指针)

```rust
  let mut values = [1, 2];
  let p1 = values.as_mut_ptr();
  let first_address = p1 as usize;
  let second_address = first_address + 4;
  let p2 = second_address as *mut i32;
  unsafe {
      *p2 += 1;
  }
  assert_eq!(values[1], 3);
```

> Rust中，强制转换并不具有传递性
> "就算e as U1 as U2"是合法的，也不能说明"e as U2"是合法的

**感谢来自creatoy的回答**，下面给出一个相关例子：

```rust
enum MyEnum {
    Zero,
}

fn main() {
    let _raw_ptr1 = MyEnum::Zero as i32 as *const i32;
    // let _raw_ptr2 = MyEnum::Zero as *const i32;
}
```

在这个例子中，`main`函数的第二行就是不合法的

##### 强制类型转换

这里有一个很有趣的问题，比如说我为`i16`类型实现了某个`Trait A`，然后`i16`是可以强制转换为`i32`的，但是事实上我们并不能说`i32`实现了`Trait A` 

##### 点操作符

> 点操作符用于访问结构体、枚举、模块、traits和其他实体中的成员/方法

> 接收器:指的是self/&self/&mut self参数

假设有一个值`value`，其类型为`T`，假设有一个方法`foo`，它有一个接收器，编译器在调用`foo`方法之前，会思考应该使用哪一个`Self`类型来调用

> 这里是在说，由于方法`foo`有一个接收器(`self`,`&self`,`&mut self`参数)，`self`指代当前实例，尽管接收器是我们在实现方法的时候指定的，即`.foo()`这个永远是`foo(value)`，但是编译器需要去看看这个`value`的类型`T`是否实现了方法`foo`，如果有，调用`T::foo()`如果没有，编译器就要去看看`&T`类型是否实现了方法`foo`，如果实现了就调用`&T::foo(value)`，在这种情况下，`value`的类型被隐式地转换为了`&T`

具体过程如下：

1. 首先编译器检查是否可以进行**值方法调用**，即直接调用`T::foo(value)`
2. 如果上一步无法完成，那么编译器会尝试**引用方法调用**，即尝试`<&T>::foo(value)`/`<&mut T>::foo(value)`
3. 如果依然不工作，那么编译器会尝试**解引用调用**，即解引用`T`，这里使用了`Deref`特征，即若`T = Deref<Target = U>`，那么编译器会尝试`U::foo(value)`
4. 若`T`不能被解引用，且`T`是一个定长类型，在编译期的类型长度已知，那么编译器会尝试将`T`转换为不定长的类型，例如将`[i32;2]`转换为`[i32]`
5. 如果还不行就报错

> 辛苦了编译器:D

### 智能指针

#### Box堆对象分配

`Box<T>`允许你将一个值分配到堆上，然后在栈上保留一个智能指针指向堆上的数据

栈的地址**从高位地址向下增长**，且栈内存是连续分配的，一般而言操作系统对栈内存大小都做出了限制，在Rust语言中，`main`线程的大小是`8MB`，普通线程的大小是`2MB`，在函数调用的时候会在**其中**创建一个临时的栈空间，调用结束后Rust会让这个栈空间里面的对象自动进入`Drop`流程，最后栈顶指针自动指向上一次的调用栈顶

与栈相反，堆上的内存则是从低位地址向高位增长，堆地址通常只受到物理内存的限制，地址分配通常也是不连续的

> `String`类型其实是智能指针结构体，存在栈上

###### 堆栈的性能

- 小型数据：在栈上分配性能和读取性能都要高于堆
- 中型数据：在栈上分配性能高，但读取性能与堆上几乎无差别，因为无法利用寄存器或CPU高速缓存，最终还是要经过一次内存的寻址
- 大型数据：只建议在堆上分配使用

栈上的分配速度在任何情况下都是优于堆的，但是读取速度往往取决于你的数据能否放入寄存器/CPU高速缓存

##### Box的使用场景

- 特意将数据放在堆上
- 数据较大的时候，在所有权转移的时候不希望发生数据内容的拷贝
- 类型的大小在编译期无法确定，但是我们需要某个固定大小的值
- 特征对象，用于说明某个对象不是特定的类型，而是实现了某个特征

Rust需要在编译期知道类型占用多大的空间，如果一种类型在编译时无法知道具体的大小，那么被称为动态大小DST

例子：递归类型、特征对象

##### Box::leak

可以消费掉`Box`并且强制目标值从内存中泄漏

```rust
fn main() {
   let s = gen_static_str();
   println!("{}", s);
}

fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}
```
我们在运行的时候得到了一个`'static`的引用，amazing！

`Box`的背后调用了`jemalloc`实现了内存管理，带`GC`语言中的对象也是借助了`Box`实现的，一切皆对象=一切皆`Box`

#### Rc与Arc实现1vN所有权机制

Rust的所有权机制要求一个值只能有一个拥有者，在大多数情况下这都没有问题，但考虑如下情况

- 图结构中，多个边可能共有一个节点，当节点没有边指向它的时候，该节点被清理释放
- 多线程中，多个线程可能共同持有一个数据，但受限于Rust的安全机制，无法同时获取该数据的可变引用

Rust在所有权机制之外引入了**引用计数**的机制允许一个数据资源在同一时刻有多个所有者，机制分为两种

- Rc机制
- Arc机制

##### Rc\<T>

通过记录一个数据被引用的次数来确定该数据是否正在被使用，当引用次数归零时就可以清除该数据

**当我们希望在堆上分配一个对象供程序的多个部分使用，并且无法确定哪个部分是最后一个使用的时候，就可以使用Rc成为数据值的所有者**

```rust
  let a = Rc::new(String::from("qwq"));
  let b = Rc::clone(&a);
  // 注意,这里的clone不是深拷贝，只是增加了Rc指针的引用计数
  assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
  // 此时Rc::strong_count(&a) = ...(&b) = 2
```

> 注意使用`Rc`智能指针的时候需要导入相应的结构体`use std::rc::Rc;`

当某个引用结束时`Rc`指针计数会减1，这得益于`Rc<T>`结构体实现了`Drop`特征

`Rc<T>`是指向数据的不可变引用，只能进行读取，如果需要修改可以考虑后面的`Mutex`与`RefCall`.

`Rc`只能用于同一线程的内部，如果需要实现线程之间的对象共享，需要使用`Arc`

`Rc<T>`实现了`Deref`特征，因此无需解引用`Rc`可以直接使用`T`

`Rc<T>`不能在线程之间安全地传递，原因在于它没有实现`Send`特征，该特征是**线程之间数据安全传输的关键**，更深层次的原因在于，`Rc<T>`需要管理计数，但是管理计数的代码中没有使用任何的**并发原语**，因此无法实现原子化的计数操作

##### Arc

`Arc`(Atomic Rc):原子化的`Rc`指针

原子化操作会带来性能损失，因此我们需要选择使用`Arc`或是`Rc`

> 这个使用几乎和`Rc`的`API`一样，只不过这个的导入是使用`use std::syna::Arc;`

---

### 多线程并发编程

> 并发是同一时间应对多件事情的能力 - Rob Pike

**并发与并行的区别**

![parallel_and_concurrent](./image/parallel_and_concurrent.png)

并发和并行都是对“多任务”处理的描述，并发是**轮流处理**，并行是**同时处理**

CPU 核心对应的是上图的咖啡机，而多个线程的任务队列就对应的多个排队的队列，由于终受限于 CPU 核心数，每个队列每次只会有一个任务被处理

**正式的定义**

如果某个系统支持两个或者多个动作的同时存在，那么这个系统就是一个并发系统。如果某个系统支持两个或者多个动作同时执行，那么这个系统就是一个并行系统。并发系统与并行系统这两个定义之间的关键差异在于 “存在” 这个词。

#### 使用多线程

##### 创建线程

> `use std::thread;`
> `use std::time::Duration;`

- 线程内部的代码需要使用闭包来执行
- `main`线程一旦结束，程序就立刻结束，因此需要保持`main`线程的存活，直到其它子线程完成自己的任务
- `thread::sleep`会让当前线程休眠指定的时间，随后其它线程会被调度运行

有可能主进程结束了子进程还没跑完，这时候我们应该用如下的代码

```rust
handle.join.unwrap();
```

这个可以阻塞`main`线程，让子线程全部跑完再来跑这个线程

必要的时候我们需要用`move`拿走变量的所有权，

##### 线程是如何结束的



---

### 宏

Rust中宏的类别：

- 声明式宏(declarative macro)/(macro_rules!)
- 过程式宏(procedural macro)
  - `#[derive]`派生宏:可以为目标**结构体**或**枚举**派生指定的代码，例如`Debug`特征
  - 类属性宏(Attribute-like macro):用于为目标添加自定义的属性
  - 类函数宏(Function-like macro):看上去就像是函数调用

##### 宏和函数的区别

- 元编程
  
  > 编写能够生成或修改代码的代码
  
  本质上宏是通过一种代码来生成另外一种代码，`println!`和`vec!`以及`#[derive(Debug)]`这些宏都可以展开成相应的代码

- 可变参数
  
  > 说起来这玩意原来只有宏能写吗，感觉C写这个也是用宏写的

- 宏展开

  由于宏会被展开成其他的代码，并且这个展开的过程是发生在编译器对代码解释之前的，所以宏可以为指定的某个类型实现某个特征：先将宏展开成实现的代码，然后再运行
  
  函数做不到这一点，因为函数需要在运行时才能被调用，但**特征需要在编译期被实现**

##### 声明式宏

声明式宏的写法和`match`的写法很像，也是一个值和一个模式相关联，**该模式会和特定的代码相关联**，并且这个**值**其实是一段代码，模式会和传入宏的代码的结构相比较，如果匹配，传入宏的代码将会被模式关联的代码(**值**)替换，最终实现宏展开

**需要注意的是，所有这些都在编译期完成，没有运行期的性能损耗**

[声明式宏文档](https://doc.rust-lang.org/reference/macros-by-example.html)

[macro_rules深入学习](https://veykril.github.io/tlborm/)

> `macro_rules`在使用的时候，定义必须在使用之前，这是因为这个玩意是在编译期做展开的，而函数不需要是因为，编译期编译器从头往下扫，不需要展开函数，而是记录函数的定义和调用关系

如果`macro_rules`被定义在模块里面的话，需要在模块前面加上`#[macro_use]`宏才可以使用

例子：

```rust
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
```

##### 用过程宏为属性标记生成代码

过程宏使用源代码作为输入参数，基于这段代码做出一系列的操作，再输出一段新的代码

> 过程宏中的`derive`宏输出的代码并不会替换之前的代码，这和`macro_rules`不一样

创建过程宏的时候，其定义需要单独放在一个Crate里面，且Crate的类型也是特殊的，这是因为如果过程宏和使用代码放在了一个Crate里面，由于过程宏是需要先编译再去使用的，使用过程宏不是一个替换的过程，那么我们就需要先对过程宏的代码编译，再对代码进行编译，但是由于**Rust的编译单元说Crate**

> 未来可能有所改变:D

---

## 常用工具链

### 自动化测试

#### 编写测试以及控制执行

当我们用cargo创建一个`lib`包的时候，它会为我们直接创建一个测试模块

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!();
    }
}
```
在这段代码中:
- `#[cfg(test)]`是一个属性宏，用于条件编译
- `test`标识测试模块
- `#[test]`是一个属性，类似于`Debug`之类的，经过`test`标记的函数就可以被测试执行器发现并执行运行，在测试模块中还可以定义一些非测试函数，用于设置环境/给测试函数提供一些通用的功能
- `you_can_assert`是我们定义的测试函数
- `assert!`一个断言

使用`cargo test`来运行所有的测试

**Rust在默认情况下会为每一个测试函数启动单独的线程去处理，当主线程`main`发现有一个测试线程失败了，`main`会将相应的测试标记为失败**

> 多线程运行测试性能高，但可能存在数据竞争的风险

在我们使用`assert!`断言时，为了更好地报错，我们可以添加更多信息
```rust
assert!(bool, "information");
```

当我们希望使用一个会`panic`的`test`的时候，我们可以在前面加上属性
```rust
#[should_panic]
```
当使用了这个之后，如果该测试函数没有按预期的`panic`那么会报错

当我们使用如下的写法的时候

```rust
#[should_panic(expected = value)]
```

这时候如果测试函数`panic`了，并且`panic`的返回值为`value`那么这个测试会通过

注意我们可以这样写，比如有一个`panic`如下
```rust
panic!("qwq {}", value);
```
那么当我们设定
```rust
#[should_panic(expected = "qwq")]
```
这样是可以通过的，换而言之，`expected`只需要识别前缀就可以了



## Appendix

### 常用Crate

#### 文件系统

[官方文档](https://doc.rust-lang.org/stable/std/fs/index.html)

这个包主要放了一些和文件系统相关的东西，包括一些结构体和函数

#### I/O

[官方文档](https://doc.rust-lang.org/stable/std/io/index.html)

这个包提供了一些I/O操作，包括读入读出之类的

### 常用方法/函数

- `parse`方法
  返回一个Result类型值，用于将字符串类型转化为整数类型

- `pop`方法
  返回一个`Option<T>`

- `ref`关键字
  可以用于将一个值绑定到一个引用上
  ```rust
  let ref a = b;
  这相当于
  let a = &b;
  ```
  这种写法在某些时候很好用，比如下面两段等价的代码
  
  **第一种写法**:

  ```rust
  fn main(){
    let y: Option<&Point> = Some(&Point { x: 100, y: 200 });
    match y {
        // 这相当是Some(value)然后把&value绑定到了q上
        Some(p) => {
            println!("Co-ordinates are {},{} ", (*p).x, (*p).y)
        }
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
  }
  ```

  **第二种写法**:

  ```rust
  fn main(){
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    match y {
        // 这相当是Some(value)然后把&value绑定到了q上
        Some(ref p) => {
            println!("Co-ordinates are {},{} ", p.x, p.y)
        }
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
  }
  ```

- `into`方法
  返回的是一个`&String`类型

- `trim`方法
  返回的是一个`&str`类型

- `to_uppercase`方法
  返回一个`String`类型

- `Clone`特征
  返回`Self`
  
- `clone`方法
  不是所有的`clone`都是深拷贝，例如Rc指针的引用计数增加，[迭代器的拷贝](https://course.rs/compiler/pitfalls/iterator-everywhere.html)
  > 判断什么时候使用`clone` -> 是否是热点路径

- `const`关键字
  需要指定类型，在编译期就可以计算出值，可以类比`#define`

- `zip`迭代器适配器
  将两个迭代器压在一起

- `filter`迭代器适配器
  可以通过闭包捕获值，然后加一个条件对迭代器中的元素进行过滤，不满足条件就从迭代器里面踢出去
  ```rust
  shoes.into_iter().filter(|s| s.size == q.size).collect();
  ```

- `fold`方法
  为实现了`Iterator`特征的类型实现的一种方法，可以从左到右对迭代器中元素做操作，最终返回一个值
  示例
  ```rust
  let a = [1, 2, 3];
  let b = a.iter().fold(0, |acc, ele| acc + ele); // 0为acc初值
  // b此时为6 即得到了1+2+3的结果
  ```

- `rfold`方法
  与`fold`方法十分类似，只不过操作顺序是从迭代器右边开始的

- `map`方法
  迭代器适配器

- `as_mut_ptr()`方法
  可以获取一个指向`slice's buffer`的`unsafe`的可变指针(裸指针)

- `try_into()`方法
  需要实现`std::convert::TryInto`特征，`try_into`方法会尝试进行一次类型转换，并返回一个`Result`类型，需要实现`TryInto`特征

- `as_mut`方法
  类型需要实现`AsMut<Type>`特征，如`String`类型可以使用是实现了`AsMut<str>`特征，返回一个`T`类型的可变引用

- `as_ref`方法
  类型需要实现`AsRef<Type>`特征，返回一个`T`类型的不可变引用

- `len`方法
  可以获取`Vec`/`HashMap`..中元素个数

- `Fromstr`特征
  当你实现了这个特征，你就可以用`parse`将某个类型转换为其他类型

- `TryFrom`特征
  一个`From`特征的安全版，`From`特征中的方法会直接返回一个值，而`TryFrom`特征会返回一个`Result`类型结果

- `Cow`智能指针
  提供不可变的引用给`borrowed data`，如果需要修改它会将数据进行`clone`操作，注意这个操作是懒惰的，使用这个需要数据实现`Borrow`特征
  > `use  std::borrow::Cow;`
  
- `env::args()`
  需要用`use std::env`引入，可以分析传入的命令行参数，并返回一个迭代器
  当传入的命令行参数中包含了非`Unicode`字符时该方法会直接崩溃

- `env::args_os()`
  返回一个`OsString`类型的迭代器，对于非`Unicode`类型的字符有更好的处理
  `arg_os`会引入额外的跨平台复杂性

- `fs::read_to_string(file_path)`
  返回一个`Result<String, Error>`类型，如果只填了文件的名字，默认路径是**项目文件**根目录下面

- `unwrap_or_else`方法
  成功返回`Ok`中的值，失败了按照后面闭包里面的处理

- `process::exit(code)`
  立即终止当前进程并返回`code`退出码，`code = 1`为非正常退出
  在多线程中，如果调用了`process::exit(code)`会直接终止所有的线程，包括正在运行和创建了还没运行的(终止整个进程)

- `line`方法
  将字符串字面量按行切割，返回一个迭代器

- `contains`方法
  接受一个参数，用于查找一个字符串中是否有这个东西

- `is_ok()`方法
  如果返回`Ok(_)`则为`True`，否则为`False`

### 随手记

- `()`这个特殊的元组作为值不能绑定到变量上

- 如果我希望访问一个`enum`类型中的字段，我可以如下访问
  ```rust
  enum qwq {
    awa,
    owo
  }

  let a = qwq::awa;
  ```

- `Result`类型中的`Err(value)`里面的`value`的类型是一个错误

- 元组没有实现迭代器特征，如果需要，可以先将元组转数组

- `cargo run -- args`
  这里的`args`就是传入的命令行参数，这里的`--`就是告诉`cargo`后面的参数不是你用的:P

- Rust中的String类型实现了`Deref Trait`，`&String`能够自动转换为`&str`

- 借助`std::env`Crate可以实现控制环境变量
  `env::var("PATH")`来读取环境变量`PATH`的值，返回一个Result类型

- 如果希望临时运行时配置环境变量可以在`cargo run`之前写，例如
  `IGNORE_CASE=1 cargo run`