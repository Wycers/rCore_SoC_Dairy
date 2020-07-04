> 我不玩 RBE 了，JOJO!

先贴个实验指导：

# 实验指导零

## 实验概要

这一章的实验指导中，你将会学到：

- 使用 Rust 包管理器 cargo 创建一个 Rust 项目
- 移除 Rust 程序对操作系统的依赖，构建一个独立化可执行的程序
- 我们将程序的目标平台设置为 RISC-V，这样我们的代码将可以在 RISC-V 指令集的裸机（Bare Metal）上执行 Rust 代码
- 生成内核镜像、调整代码的内存布局并在 QEMU 模拟器中启动
- 封装如输出、关机等一些 SBI 的接口，方便后续开发

先很普通地建个项目，指定了 toolchain 为 nightly-2020-06-27，据说是用了很多新的特性……

默认创建的 Hello World 程序长这样：

```rust

fn main() {
    // println!("Hello, world!");
}

```

其实它默认依赖了 std，它会依赖 os 的一些功能。但为了从头开发 os，我们需要移除这个程序对 os 的依赖。

所以我们先禁用掉 std，但是这样同时会导致 panic_handler 无法实现，所以只能自己实现它；另外这同时会导致 panic 的时候用于处理堆栈跟踪的语义项 eh_personality 失效。

这里为了简单起见，禁用掉 std 以及在 panic 时直接终止。
main.rs:

```rust
#![no_std]
...
```

os/Cargo.toml:

```

...

# panic 时直接终止，因为我们没有实现堆栈展开的功能

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

```

```

```
