# rCore_SoC_Dairy

## 回顾

- 完成了一个最小的 riscv os 镜像，并在 qemu 上运行。
- 完成了 RISC-V 中有关中断处理的部分。

## 今日

要开始内存管理了。

为了方便起见，使用现成的`buddy_system_allocator`

``` yaml
# Cargo.toml

[dependencies]
...
buddy_system_allocator = "0.3.9"
```



看着一个 os 从零到开始，真的真的好开心啊。
