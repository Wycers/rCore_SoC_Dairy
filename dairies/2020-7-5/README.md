# rCore_SoC_Dairy

## 回顾

```rust

//! # 全局属性
//! - `#![no_std]`
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]

use core::panic::PanicInfo;

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// 覆盖 crt0 中的 _start 函数
/// 我们暂时将它的实现为一个死循环
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

```

- 为了不依赖 OS， 使用`no_std`移除了默认使用的标准库。
- 使用`panic_handler`，实现移除了标准库后缺失的错误处理。
- 使用`no_mangle`，将程序的入口点固定为`_start`
- extern "C" 可以将`_start`导出为 C 风格的函数。
- 这样以后，main 函数已经失去作用。

## 今日

执行`cargo build`，会出现一大段链接错误，这是因为系统的链接器会默认链接当前系统的 C 运行时环境，使用特定的链接器参数可以 Abort 这个行为，但是这边我们考虑直接编译成裸机目标。

```bash
rustup target add riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf
```

这样就可以将项目编译成使用 risc-v 指令集的无操作系统可执行文件。

为了避免多次使用`--target`参数的麻烦，我们将其保存为配置文件。

os/.carge/config

```plain
# 编译的目标平台
[build]
target = "riscv64imac-unknown-none-elf"
```

为了分析编译产物，我们可以使用 rust 社区提供的工具集

```
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

安装成功之后，可以通过

```
rust-objdump target/riscv64imac-unknown-none-elf/debug/os -x --arch-name=riscv64
```

分析其信息。在本人环境下，这里为：

```plain
target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv

architecture: riscv64
start address: 0x0000000000011120

Program Header:
    PHDR off    0x0000000000000040 vaddr 0x0000000000010040 paddr 0x0000000000010040 align 2**3
         filesz 0x00000000000000e0 memsz 0x00000000000000e0 flags r--
    LOAD off    0x0000000000000000 vaddr 0x0000000000010000 paddr 0x0000000000010000 align 2**12
         filesz 0x0000000000000120 memsz 0x0000000000000120 flags r--
    LOAD off    0x0000000000000120 vaddr 0x0000000000011120 paddr 0x0000000000011120 align 2**12
         filesz 0x0000000000000004 memsz 0x0000000000000004 flags r-x
   STACK off    0x0000000000000000 vaddr 0x0000000000000000 paddr 0x0000000000000000 align 2**64
         filesz 0x0000000000000000 memsz 0x0000000000000000 flags rw-

Dynamic Section:
Sections:
Idx Name            Size     VMA              Type
  0                 00000000 0000000000000000
  1 .text           00000004 0000000000011120 TEXT
  2 .debug_str      0000041a 0000000000000000
  3 .debug_abbrev   00000113 0000000000000000
  4 .debug_info     0000053c 0000000000000000
  5 .debug_aranges  00000040 0000000000000000
  6 .debug_ranges   00000030 0000000000000000
  7 .debug_pubnames 000000a4 0000000000000000
  8 .debug_pubtypes 00000308 0000000000000000
  9 .debug_frame    00000050 0000000000000000
 10 .debug_line     0000005b 0000000000000000
 11 .comment        00000013 0000000000000000
 12 .symtab         00000108 0000000000000000
 13 .shstrtab       000000a5 0000000000000000
 14 .strtab         0000002d 0000000000000000

SYMBOL TABLE:
0000000000000000 l    df *ABS*  00000000 4h6htaxfpfwuroyu
0000000000011120         .text  00000000
0000000000011120         .text  00000000
0000000000011120         .text  00000000
0000000000011124         .text  00000000
0000000000000000         .debug_ranges  00000000
0000000000000000         .debug_info    00000000
0000000000000000         .debug_frame   00000000
0000000000000000         .debug_line    00000000 .Lline_table_start0
0000000000011120 g     F .text  00000004 _start
```

这其中：

- start address：程序的入口地址
- Sections：从这里我们可以看到程序各段的各种信息。后面以 debug 开头的段是调试信息
- SYMBOL TABLE：符号表，从中我们可以看到程序中所有符号的地址。例如 \_start 函数就位于入口地址上
- Program Header：程序加载时所需的段信息，其中的 off 是它在文件中的位置，vaddr 和 paddr 是要加载到的虚拟地址和物理地址，align 规定了地址的对齐，filesz 和 memsz 分别表示它在文件和内存中的大小，flags 描述了相关权限（r 表示可读，w 表示可写，x 表示可执行）

可以通过

```
rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64
```

进行反编译，获得程序的程序码

```
target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv


Disassembly of section .text:

0000000000011120 _start:
   11120: 09 a0                         j       2
   11122: 01 a0                         j       0
```

就是一个死循环，查询[ASM](https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md)看到 ASM 命令的含义

那先这样吧。

接下来，删去这个 elf 中的冗余信息（符号表、调试信息等），作出一个系统镜像

```
rust-objcopy target/riscv64imac-unknown-none-elf/debug/os --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/kernel.bin
```

之后，我们还需要调整内存布局，因为系统内核一般从高地址开始，用户态程序从低地址开始。为了调整内存布局，我们生成一个链接器脚本

```
/* 有关 Linker Script 可以参考：https://sourceware.org/binutils/docs/ld/Scripts.html */

/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

/* 数据存放起始地址 */
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . 表示当前地址（location counter） */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }

    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }

    data_start = .;

    /* .data 字段 */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }

    bss_start = .;

    /* .bss 字段 */
    .bss : {
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
    }

    /* 结束地址 */
    kernel_end = .;
}
```

并且在.cargo/config 里指定使用这个链接器脚本

`build`后再次使用`-d`反编译，可以看见内存地址已经正确放置了。

```
target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv


Disassembly of section .text:

0000000080200000 text_start:
80200000: 09 a0                         j       2
80200002: 01 a0                         j       0
```
