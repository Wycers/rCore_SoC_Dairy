# rCore_SoC_Dairy

# 2020-10-2

今天来fix之前没有做完的lab2。


## 物理页

通常来说，我们将连续的4KB内存作为一个物理页，作为一个物理内存分配的基本单位。我们可以通过物理页号（PPN），来得到其对应的物理地址为 $[\text{PPN} \times 4\text{KB}, \text{PPN} \times 4\text{KB} + 4\text{KB}]$的一段物理内存空间。

在这种物理内存映射规则下，对于某一物理地址$\text{PA}$，其所在的物理页的页号为$\text{PA} / 4\text{KB}$，或者$\text{PA} >> 12$。

我们用**Frame Allocat**和**Frame Tracker**来实现物理页的分配和回收。

其中**Frame Tracker**是一个其物理地址的标识，用这个对象可以方便地获得物理页号，也可以随着不再需要这个物理页，我们需要回收，我们利用 Rust 的 drop 机制在析构的时候自动实现回收。

```rust
impl FrameTracker {
    /// 帧的物理地址
    pub fn address(&self) -> PhysicalAddress {
        self.0
    }
    /// 帧的物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0)
    }
}
```

这边没有按教程的做法将分配算法放在`algorithm`目录下，而是将物理页相关的内容全部放在`memory/frame`下。。



