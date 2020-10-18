//! # 全局属性
//! - `#![no_std]`
//!   禁用标准库
#![no_std]

//!
//! - `#![no_main]`
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]

//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(asm)]`
//!   内嵌汇编
#![feature(asm)]

//!
//! - `#![feature(global_asm)]`
//!   内嵌整个汇编文件
#![feature(global_asm)]

//!
//! - `#![feature(panic_info_message)]`
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
#![feature(llvm_asm)]
#![feature(alloc_error_handler)]

extern crate alloc;

use os::interrupt;
use os::memory;
use os::println;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("asm/entry.asm"));
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 初始化各种模块
    interrupt::init();
    memory::init();

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped");

    extern "C" {
        fn __restore(context: usize);
    }
    // 获取第一个线程的 Context，具体原理后面讲解
    let context = PROCESSOR.lock().prepare_next_thread();
    // 启动第一个线程
    unsafe { __restore(context as usize) };

    loop {}

    unreachable!()
}
