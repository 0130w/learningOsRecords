# RISC-V架构学习笔记

## 目录

- [RISC-V架构学习笔记](#risc-v架构学习笔记)
  - [目录](#目录)
  - [Computer Abstractions and Technology](#computer-abstractions-and-technology)
    - [Eight Great Ideas in Computer Architecture](#eight-great-ideas-in-computer-architecture)
    - [Technologies for Building Rpocessors and Memory](#technologies-for-building-rpocessors-and-memory)
    - [Performance](#performance)
      - [Measuring Performance](#measuring-performance)
      - [CPU性能及其度量因素](#cpu性能及其度量因素)
      - [指令性能](#指令性能)
      - [经典的CPU性能公式](#经典的cpu性能公式)
    - [从单处理器到多处理器](#从单处理器到多处理器)
      - [评测CPU性能](#评测cpu性能)
    - [谬误与陷阱](#谬误与陷阱)
  - [Instruction:Language of the computer](#instructionlanguage-of-the-computer)

## Computer Abstractions and Technology

### Eight Great Ideas in Computer Architecture

- Design for Moore's law

### Technologies for Building Rpocessors and Memory

芯片从制造到用户的过程

![chip2user](./image/chip2user.png)

良率(yield):合格芯片数占总芯片数的百分比，上述的第五步

### Performance

- 响应时间(execution time):计算机完成某个任务所需要的总时间，包括硬盘访问，内存访问，I/O活动，操作系统开销，CPU执行时间等
- 吞吐率，也叫带宽(bandwidth):单位时间内能完成的任务数量

> 个人用户通常对响应时间更感兴趣，数据中心对管理者感兴趣的通常是吞吐率

> 并不是说改善响应时间就一定会改善吞吐率，比如某个任务需要的总时间减少了，但是可能是计算机将更多的资源分配给了这个任务，从而导致了实际上吞吐率的下降，反之亦然

前几章测量性能的时候主要考虑的是响应时间，对于某个计算机X我们可以有如下的性能计算公式

$$Performance_X = \frac{1}{Execution\ time_X}$$

#### Measuring Performance

时间是计算机性能的衡量标准：完成同样的计算任务，需要时间最少的计算机是最快的.但时间也有不同的定义方法，最直接的定义叫做挂钟时间`Wall clock time`，又叫响应时间`response time`，运行时间`elapsed time`等

- CPU执行时间：执行某一任务在CPU上所花费的时间，不包括I/O等待或运行其他程序的时间

CPU时间还可以进一步分为用于用户程序的时间和操作系统为用户程序执行相关程序所花去的CPU时间，前者被称为**用户CPU时间**，后者称为**系统CPU时间**

- 系统性能：本书中表示空载系统的响应时间
- CPU性能：本书中表示用户CPU时间

#### CPU性能及其度量因素

$$CPU\ execution\ time = CPU\ clock\ cycles\ for\ a\ program\ \times\ Clock\ cycle\ time$$

硬件设计者为了减少程序执行所需的CPU时钟周期数或缩短时钟周期长度，就能改进性能.但我们时常需要面对这两者的抉择，许多技术在减少程序CPU时钟周期数时会增加时钟周期长度

#### 指令性能

对于一个程序而言，编译器生成了明确的指令，且计算机必须通过指令来运行程序，因此执行时间依赖于程序中的指令数

一种考虑方法为

$$CPU\ clock\ cycles = Instructions\ for\ a\ program \times Average\ clock\ cycles\ per\ instruction$$

然后再结合前面的**CPU性能公式**

指令平均时钟周期数(clock cycle per instruction)：表示执行每条指令所需的时钟周期平均数，缩写为`CPI`

#### 经典的CPU性能公式

$$CPU\ execution\ time = CPI\ \times Instruction\ count \times Clock\ cycle\ time$$

**理解程序性能**：程序的性能与算法、编程语言、编译器、体系结构以及实际的硬件有关

![performanceEffection](/image/performanceEffection.png)

有些处理器在每个时钟周期可以取多条指令并执行，有些设计者用IPC来代替指令平均执行周期数CPI，如一个处理器每个时钟周期可以处理2条指令，那么它的IPC为2，CPI为0.5

时钟周期长度一般而言是固定的，但是为了节省能量/暂时提升性能，今天的计算机可以使用不同的时钟频率，因此我们对程序需要使用**平均时钟频率**(锁频和超频)

现代集成电路技术中占主导地位的是CMOS(互补型金属氧化半导体)，其主要的能耗来源是动态能耗，即在晶体管开关过程中产生的能耗，即晶体管的状态从0到1，从1到0的过程消耗的能量

动态能耗取决于每个晶体管的负载电压和工作电压

$$Energy\varpropto Capacitive\ \times load Voltage^2$$

这个式子表示的是一次$0\to 1\to 0$或是$1\to 0\to 1$的逻辑转换消耗的能量

一个晶体管消耗的能量

$$Energy\varpropto \frac{1}{2} \times Capacitive\ \times load Voltage^2$$

那么每个晶体管的功耗为

$$Power\varpropto \frac{1}{2}\times Capacitive\ \times load Voltage^2 \times Frequency\ switched$$

其中，开关频率是时钟频率的函数，负载电容是连接到输出上的晶体管数量(称为扇出fanout)和工艺的函数.每次工艺更新换代时都会降低电压，从而达到大幅度减少功率和能耗

但电压是不能一直下降的，电压的下降会导致晶体管的泄漏电流过大，目前40%的功耗都是由于泄漏电流造成的

> CMOS的主要功耗来自于动态能耗，但是静态能耗也是存在的，及时在晶体管关闭的情况下，也有泄漏电流存在.在服务器中，典型的泄漏电流占40%的能耗

### 从单处理器到多处理器

为了减少处理器(processor)和微处理器(microprocessor)之间的语义之间的混淆，一些公司将处理器作为核(core)的代称，在这种语境下，微处理器(microprocessor)就是多核处理器

显式并行程序的编写困难的原因
- 并行程序以提高性能为目的，这必然增加编程的难度
- 为了充分发挥并行硬件的速度，程序员必须将应用划分为每个core上有大致相同数量的任务，并同时完成.与此同时还需要减少调度的开销，不浪费并行的性能

#### 评测CPU性能

为什么SPEC要用几何平均值?
    这更公平，能够让一个坏的结果不是简单的被一个好的结果所抵消，且它受极端值的影响较小

### 谬误与陷阱

以下为正确的叙述

- 1.改进计算机的某个方面总性能的提高与改进大小不成正比
  考虑Amdahl定律

- 2.低利用率的计算机其功耗的降低不一定是显著的

- 3.不要用性能公式的部分方面去度量性能

- 4.面向性能的设计与面向能效的设计具有相关的目标
  能耗是功率和时间的乘积，即使我们为了提高性能而增大了功率，但因此也减少了时间，所以有可能能耗其实是减少的

## Instruction:Language of the computer