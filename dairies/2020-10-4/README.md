# rCore_SoC_Dairy

# 2020-10-4

今天来为这个 os 添加虚拟内存的机制。到目前为止这个操作系统还只有一个内核程序，没有常见意义上的用户程序运行。
为了方便地支持用户进程以及多任务切换，我们需要为 os 提供虚拟内存机制，并将之前直接运行在物理内存上的 os 切换到虚拟内存空间上运行。

## risc-v 的 sv39 分页方案

![image-20201004211520298](C:\Users\Wycer\AppData\Roaming\Typora\typora-user-images\image-20201004211520298.png)

![image-20201004210400344](C:\Users\Wycer\AppData\Roaming\Typora\typora-user-images\image-20201004210400344.png)

sv39 中的 39 指的是虚拟地址的长度为 39bit。其对应的页表项中 PPN 的字段长 44bit，以支持 56bit 的物理地址。同时，为了使得页表大小和页面大小一致为 4KB，其使用基数为 2^9^的三级页表。

## 部署 sv39

### 修改内核

1. 将链接器中使用的基址改为虚拟地址，加上内存对齐防止一页内有多个段。
2. 对应修改 `os/src/memory/config.rs` 中的 `KERNEL_END_ADDRESS` 修改为虚拟地址并加入偏移量。
3. 在 asm 中加载一个初始的页表。
   1. 这个初始的页表中的第 510 项为`0x80000 << 10 | 0xcf`，表示虚拟地址`0xffff_ffff_8000_0000`映射到`0x8000_0000`。`0xffff_ffff_8000_0000`中的`VPN[2]=0b1_1111_1110`，即`dec(510)`。`0xcf` 表示 VRWXAD 均为 1。
   2. 保留 `0x8000_0000` -> `0x8000_0000` 的映射原因是诸如 `PC` 之类的寄存器，在系统启动时即被使用，内部的地址仍为单纯的物理地址，在跳转`rust_main`之前均使用这样的物理地址，所以需要保留一个原始映射。
   3. 刷新 TLB。

### 实现页表

思路是将一个分配好的物理页作为页表，页表中的每一项都是 8 字节的页表项。页表项实际上就是对 usize（8 字节）的封装。

### 内存重映射

在使用内存的时候，可以注意到在一个 section 里的、一些连续的内存往往有着相同的 rwx 权限。因此我们可以把这样的一片内存抽象成一个内存段。同时可以为不同的内存段提供不同的映射方案。

```rust
/// 映射的类型
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    /// 线性映射，操作系统使用
    Linear,
    /// 按帧分配映射
    Framed,
}

/// 一个映射片段（对应旧 tutorial 的 `MemoryArea`）
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    /// 映射类型
    pub map_type: MapType,
    /// 所映射的虚拟地址
    pub range: Range<VirtualAddress>,
    /// 权限标志
    pub flags: Flags,
}
```

随后

```rust
/// 某个线程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}
```

维护一个这样的数据结构来记录某个线程的内存映射关系。在查找页表项的时候如果查找到空项，就为其分配新页表、将新页表写入页表项、保存新页表。

然而对于用户程序而言，它需要知道自己的各种段的储存位置，所以我们需要另一个数据结构来记录某个程序的各个区段在内存中的位置、分配的页。

```rust
pub struct MemorySet {
    pub mapping: Mapping,
    pub segments: Vec<Segment>,
}
```
