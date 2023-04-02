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
    - [Crate和Modules](#crate和modules)
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
  - [Appendix](#appendix)
    - [常用Crate](#常用crate)
      - [文件系统](#文件系统)
      - [I/O](#io)
    - [常用方法/函数](#常用方法函数)


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
    // 这里用 pub components : Vec<&dyn Draw>就不太行
    // 因为要指明声明周期
}
```

如果使用泛型，我们可以用如下的写法，但这就会导致Vec中的所有都是同一个类型的数据，而上面使用特征对象的写法就不需要Vec中的都是同一类型的数据
```rust
pub struct Screen<T : Draw> {
        pub components : Vec<T>,
}
```

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

### Crate和Modules

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

**限制性语法**

```rust
pub //可见性无限制
pub(crate)  //在当前包内可见
pub(self) //在当前模块可见
pub(super)  //在父模块可见
pub(in <path>)  //在某个路径的模块可见，其中<path>必须是父模块/祖先模块
```

> 在同一个模块中一个模块项可以访问其他的模块项，即使是私有的

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

## Appendix

### 常用Crate

#### 文件系统

> https://doc.rust-lang.org/stable/std/fs/index.html

这个包主要放了一些和文件系统相关的东西，包括一些结构体和函数

#### I/O

> https://doc.rust-lang.org/stable/std/io/index.html

这个包提供了一些I/O操作，包括读入读出之类的

### 常用方法/函数

- `parse`方法
  返回一个Result类型值，用于将字符串类型转化为整数类型