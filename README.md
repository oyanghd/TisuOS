> 未经本人许可，不许用于任何商业用途

# 太素 TisuOS

gitee 地址：https://gitee.com/belowthetree/tisu-os

github 地址：https://github.com/belowthetree/TisuOS

## 编译

环境：

* wsl
* riscv64-unknown-linux-gnu-gcc
* rust-nightly

指令：`make all`

注意：GitHub 版本没有硬盘镜像，需要自己创建一个 FAT32 格式的硬盘镜像

## 运行：

* QEMU-system-riscv64

`qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 128M -drive if=none,format=raw,file=hdd.dsk,id=foo -device virtio-blk-device,scsi=off,drive=foo -nographic -serial mon:stdio -bios none -device virtio-rng-device -device virtio-gpu-device -device virtio-net-device -device virtio-tablet-device -device virtio-keyboard-device -kernel `

## 内核功能

### Uart

通用异步收发传输器

命令行的输入可以通过 MMIO 获取，同时可以输出到命令行，作为主要的调试手段

### 中断

包括汇编、Rust 两部分，中断首先跳转到汇编部分，再转到 Rust 部分

汇编部分负责保存与恢复环境，中断负责具体的处理

### 内存管理

* 分页管理，将内存按照 4kb 大小的页表进行记录管理，前 1G 的地址属于 MMIO。128MB 内存，内核分到 80MB
* 分块堆内存管理，在分页管理的基础上，采用类似 SLAB 的方法，每次分配内存时将大小对齐到 2^n，将申请到的内存按照对应大小划分成许多块进行标记管理。

### 页表映射

在分页管理的基础上，管理三级的页表。仅在映射时进行内存申请。

### PLIC

平台级中断控制

主要用于控制时钟中断和软件中断（待完成）

* 时钟中断，现设置为 15_0000 个周期（15ms）进行一次中断

### 同步

利用 RISCV 原子交换实现

* 单重锁，仅可以同时上锁一次
* 多重锁，同一个核心可以上锁多次（主要是给中断时的进程调度使用，可以在函数调用中多次上锁）
* 读写锁，支持读取时允许多次上锁，写入时只允许锁一次

### 任务

* 进程，分为用户、内核两种，作为程序存在的标志
  * 内存映射，默认映射执行地址的一个页表以及栈内存对应页表。同时为内核进程映射所有内核用地址以及用户地址，为用户进程映射内核系统调用地址
  * 进程代表程序，管理所属的所有线程
* 线程，作为进程的子集，通过 fork 产生，更加轻量，不需要再次进行内存映射
  * 每个线程分配16张页表作为栈
  * 调度，通过时钟中断触发调度，暂停当前内核标记为 Running 的进程、选取下一个标记为 Waiting 的进程标记为 Running 并运行
  * 第一个线程作为主线程，主线程结束所有线程被卸载，进程结束

### 外部设备

* 块设备，即磁盘。支持同步读写（用单重锁实现，操作发出后锁住，读写完成后触发中断解锁）、异步读写（创建一个 Sleeping 的进程，读写完成后触发中断唤醒待执行进程）
* GPU，图形化显示
* 输入设备，包括键盘、鼠标等的输入（nographic 选项下无效，因为需要 qemu 的窗口捕捉输入）

### 磁盘读写缓冲

预先读取大块磁盘中的内容放置在缓冲内存中。所有读写操作都写入缓冲部分内存。目前只实现了读取，写入待完成。

### 文件系统

目前实现了 FAT32 文件系统的读取。

文件系统以文件树的形式展示，可以读取运行 elf 程序

### Shell（调试用）

一个简单的命令行交互程序，以独立进程运行，用于调试功能

### 图形接口

* 底层显示维护一个矩形作为显示内容，支持透明度，更多功能待添加

